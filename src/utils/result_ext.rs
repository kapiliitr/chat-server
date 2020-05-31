use models::ChatErrorKind::{
    ConfigInitializationFailed, IOOperationFailed, ServerAddressParseFailed,
};
use models::{ChatError, ChatResult};
use std::net::AddrParseError;

pub trait ResultExt<T> {
    fn from_err(self) -> ChatResult<T>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    ChatError: From<E>,
{
    fn from_err(self) -> ChatResult<T> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(e.into()),
        }
    }
}

impl From<envconfig::Error> for ChatError {
    fn from(err: envconfig::Error) -> Self {
        ChatError::new_with_error(ConfigInitializationFailed, err)
    }
}

impl From<AddrParseError> for ChatError {
    fn from(err: AddrParseError) -> Self {
        ChatError::new_with_error(ServerAddressParseFailed, err)
    }
}

impl From<std::io::Error> for ChatError {
    fn from(err: std::io::Error) -> Self {
        ChatError::new_with_error(IOOperationFailed, err)
    }
}
