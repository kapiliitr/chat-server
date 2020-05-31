use std::fmt::Debug;

pub type ChatResult<T> = std::result::Result<T, ChatError>;

#[derive(Debug, Display)]
#[display(fmt = "{}, details: {}", error_kind, message)]
pub struct ChatError {
    pub error_kind: ChatErrorKind,
    pub message: String,
}

#[derive(Debug, Display)]
pub enum ChatErrorKind {
    #[display(fmt = "Failed to initialize server config")]
    ConfigInitializationFailed,
    #[display(fmt = "Failed to parse server address")]
    ServerAddressParseFailed,
    #[display(fmt = "Failed to perform IO operation")]
    IOOperationFailed,
    #[display(fmt = "Chat already exists for the participants")]
    ChatAlreadyExists,
    #[display(fmt = "No chats found for the user")]
    NoChatsFound,
    #[display(fmt = "A chat with the specified id does not exist")]
    ChatDoesNotExist,
}

impl std::error::Error for ChatError {}

impl ChatError {
    pub fn new(error_kind: ChatErrorKind) -> Self {
        ChatError {
            error_kind,
            message: String::new(),
        }
    }

    pub fn new_with_error<T>(error_kind: ChatErrorKind, inner_error: T) -> Self
    where
        T: Debug,
    {
        ChatError {
            error_kind,
            message: format!("{:?}", inner_error),
        }
    }
}
