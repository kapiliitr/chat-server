use mio::net::TcpListener;
use mio::{Event, Events, Poll, PollOpt, Ready, Token};
use models::{ChatError, ChatResult};
use server::{Connection, ServerConfig};
use slab::Slab;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::net::SocketAddr;
use utils::ResultExt;
use {requests, utils};

const SERVER_TOKEN: usize = 0;

pub struct Server {
    pub socket: TcpListener,
    pub token: Token,
    pub connections: Slab<Connection>,
}

impl Server {
    pub fn run(config: ServerConfig) -> ChatResult<()> {
        let mut server = Server::new(config)?;
        let poll = Poll::new().from_err()?;

        poll.register(
            &server.socket,
            server.token,
            Ready::readable(),
            PollOpt::edge(),
        )
        .from_err()?;

        let mut events = Events::with_capacity(1024);
        loop {
            poll.poll(&mut events, None).from_err()?;

            for event in &events {
                server.handle_event(&poll, event)
            }
        }
    }

    fn new(config: ServerConfig) -> ChatResult<Self> {
        let conn_string = format!("{}:{}", config.hostname, config.port);
        info!("Starting server at address {}", conn_string);
        conn_string
            .parse::<SocketAddr>()
            .from_err()
            .and_then(|address| TcpListener::bind(&address).from_err())
            .and_then(|socket| {
                Ok(Server {
                    socket,
                    token: Token(SERVER_TOKEN),
                    connections: Slab::with_capacity(config.max_num_connections),
                })
            })
    }

    fn handle_event(&mut self, poll: &Poll, event: Event) -> () {
        match event.token() {
            Token(SERVER_TOKEN) => {
                if self.connections.len() >= self.connections.capacity() {
                    warn!("Unable to accept connection due to maximum load");
                    return;
                }
                self.handle_server_event(&poll, event)
            }
            client_token => {
                let client_key = utils::get_key_from_client_token(client_token);
                if !self.connections.contains(client_key) {
                    warn!("Did not find connection for token {:?}", client_token);
                    return;
                }
                self.handle_client_event(&poll, event, client_key)
            }
        }
    }

    fn handle_server_event(&mut self, poll: &Poll, event: Event) -> () {
        loop {
            match self.socket.accept() {
                Ok((client_socket, client_address)) => {
                    info!("Received connection from {}", client_address);
                    let entry = self.connections.vacant_entry();
                    let key = entry.key();
                    let client_token = utils::get_client_token_from_key(key);
                    poll.register(
                        &client_socket,
                        client_token,
                        Ready::readable(),
                        PollOpt::edge(),
                    )
                    .and_then(|_| {
                        entry.insert(Connection {
                            socket: client_socket,
                            address: client_address,
                            token: client_token,
                            pending: None,
                        });
                        Ok(())
                    })
                    .unwrap_or_else(|err| {
                        self.connections.remove(key);
                        let e: ChatError = err.into();
                        error!("{}", e);
                    });
                }
                Err(ref err) if err.kind() == io::ErrorKind::WouldBlock =>
                // No more connections ready to be accepted
                {
                    break;
                }
                Err(err) => {
                    let e: ChatError = err.into();
                    error!("{}", e);
                }
            }
        }
    }

    fn handle_client_event(&mut self, poll: &Poll, event: Event, key: usize) -> () {
        debug!("Client event received {:?}", event);

        if event.readiness().is_readable() {
            self.handle_readable_client_event(poll, key)
        } else if event.readiness().is_writable() {
            self.handle_writable_client_event(poll, key)
        }
    }

    fn handle_readable_client_event(&mut self, poll: &Poll, key: usize) -> () {
        let conn = unsafe { self.connections.get_unchecked_mut(key) };
        let mut stream_reader = match conn.socket.try_clone() {
            Ok(read_stream) => BufReader::new(read_stream),
            Err(err) => {
                warn!("Failed to clone tcp stream for reading");
                return self.close_connection_on_error(key, err);
            }
        };
        let mut request_string = String::new();

        loop {
            match stream_reader.read_to_string(&mut request_string) {
                Ok(0) => {
                    debug!("Read 0 bytes, socket closed");
                    // Socket is closed, remove it from the map
                    self.close_connection(key);
                    break;
                }
                Ok(size) => {
                    debug!("Read {} bytes, {:?}", size, request_string);
                }
                Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
                    debug!("Request received {}", request_string);
                    debug!("Received WouldBlock");
                    {
                        conn.process_request(&request_string);
                    }
                    request_string.clear();

                    poll.reregister(
                        &conn.socket,
                        conn.token,
                        Ready::writable(),
                        PollOpt::edge() | PollOpt::oneshot(),
                    )
                    .unwrap_or_else(|err| {
                        self.close_connection_on_error(key, err);
                    });
                    // Socket is not ready anymore, stop reading
                    break;
                }
                Err(err) => {
                    self.close_connection_on_error(key, err);
                    break;
                }
            }
        }
    }

    fn handle_writable_client_event(&mut self, poll: &Poll, key: usize) -> () {
        let conn = unsafe { self.connections.get_unchecked_mut(key) };
        debug!("Writing to socket");
        let mut stream_writer = match conn.socket.try_clone() {
            Ok(write_stream) => BufWriter::new(write_stream),
            Err(err) => {
                warn!("Failed to clone tcp stream for writing");
                return self.close_connection_on_error(key, err);
            }
        };
        if let Some(ref response) = conn.pending {
            if let Err(err) = response
                .iter()
                .try_for_each(|line| stream_writer.write(line.as_bytes()).map(|num| ()))
            {
                warn!("Failed to write response to buffer");
                return self.close_connection_on_error(key, err);
            }
            match stream_writer.flush() {
                Ok(_) => {
                    debug!("Write completed, closing connection");
                    // Socket is closed, remove it from the map
                    self.close_connection(key);
                }
                Err(err) => self.close_connection_on_error(key, err),
            }
        }
    }

    fn close_connection(&mut self, key: usize) {
        self.connections.remove(key);
    }

    fn close_connection_on_error<E>(&mut self, key: usize, err: E)
    where
        ChatError: From<E>,
    {
        self.close_connection(key);
        let e: ChatError = err.into();
        error!("{}", e);
    }
}
