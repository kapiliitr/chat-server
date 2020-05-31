use mio::Token;

pub fn get_key_from_client_token(token: Token) -> usize {
    token.0 - 1
}

pub fn get_client_token_from_key(key: usize) -> Token {
    Token(key + 1) // Because 0 is reserved for SERVER_TOKEN
}
