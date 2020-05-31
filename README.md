# Simple Chat Server

A minimal implementation of a chat server with limited capabilities, designed and developed for learning and educational purposes only. [NOT FOR PRODUCTION USE]

## Code structure

`main.rs` is the entry point of the server application.

`models` contains the declaration of data structures used by the server.

`store` contains the interface of the storage layer used by the server. Currently, it is implemented in-memory. 

`requests` contains the implementation of handling of each requests supported by the server.

`utils` contains helper methods used by the server.

`core` contains the implementation of the server.

## How-To Guide

### Prerequisites

1. Rust Programming Language
2. Cargo
3. Java Runtime

### Build and Run

1. Clone the repository and run `cd signal`

2. Set the following environment variables. Provided below are their default values-<br>

    >SERVER_HOSTNAME = 127.0.0.1
    >
    >SERVER_PORT = 8080
    >
    >MAX_NUM_CONNECTIONS = 10

3. Run `cargo build`

4. Run `cargo run --color=always --package chat_server --bin chat_server` which will start the server.

5. Run the following command to send requests to the server after replacing the parameters with appropriate values.<br>
    ```
   java -jar rest_api/message-tools.jar \
   -c <number of messages to generate> \
   -h=<the destination host> \
   -p=<the destination port> \
   generate-requests
   ```

6. Alternatively, you can also use `curl` or tools like `Postman` to send HTTP requests to the server. Some sample requests are as follows-
   ##### Creates a chat between two users.
   ```
    curl -d '{"id":17692,"participantIds":[100250,51549]}' -H "Content-Type: application/json" -X POST http://localhost:8080/chats
    ```
   
   ##### Adds a message to a chat.
   ```
   curl -d '{"id":"d9a0360a-c60c-48e3-aa74-c637286d8eae","timestamp":1574614010723,"message":"The harder he tried the less he got done.","sourceUserId":100250,"destinationUserId":51549}' -H "Content-Type: application/json" -X POST http://localhost:8080/chats/17692/messages
   ```
   
   ##### Lists a user’s current chats.
   ```
   curl -X GET http://localhost:8080/chats?userId=51549
   ```
      
   ##### Lists a chat’s messages.
   ```
   curl -X GET http://localhost:8080/chats/17692/messages
   ```
      
### To-Do

1. User registration and management of contact list.
2. Ordering of chat messages chronologically.
3. Add persistent storage.
4. Add test coverage.
   