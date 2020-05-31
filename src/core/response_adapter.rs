use models::{ChatError, ChatErrorKind};
use serde::Serialize;

pub fn create_success_response<T>(method: &str, data: T) -> Vec<String>
where
    T: Serialize,
{
    let status = match get_success_status_code(method) {
        200 => "200 OK",
        201 => "201 CREATED",
        _ => "500 INTERNAL SERVER ERROR",
    };
    let message = serde_json::to_string(&data).unwrap();
    vec![
        format!("HTTP/1.1 {}\r\n", status),
        "Content-Type: application/json\r\n".to_string(),
        format!("Status: {}\r\n\r\n", status),
        format!("{}\r\n", message),
    ]
}

pub fn invalid_response() -> Vec<String> {
    let status = "400 BAD REQUEST";
    let message = "Invalid request received";
    vec![
        format!("HTTP/1.1 {}\r\n", status),
        format!("Status: {}\r\n\r\n", status),
        format!("{{\"error\":\"{}\"}}\r\n", message),
    ]
}

pub fn create_error_response(err: ChatError) -> Vec<String> {
    let status = match get_error_status_code(&err.error_kind) {
        404 => "404 NOT FOUND",
        409 => "409 CONFLICT",
        _ => "500 INTERNAL SERVER ERROR",
    };
    vec![
        format!("HTTP/1.1 {}\r\n", status),
        format!("Status: {}\r\n\r\n", status),
        format!("{{\"error\":\"{}\"}}\r\n", err),
    ]
}

fn get_success_status_code(method: &str) -> u32 {
    match method {
        "POST" => 201,
        "GET" => 200,
        _ => 500,
    }
}

fn get_error_status_code(error_kind: &ChatErrorKind) -> u32 {
    match error_kind {
        ChatErrorKind::ConfigInitializationFailed => 500,
        ChatErrorKind::ServerAddressParseFailed => 500,
        ChatErrorKind::IOOperationFailed => 500,
        ChatErrorKind::ChatAlreadyExists => 409,
        ChatErrorKind::NoChatsFound => 404,
        ChatErrorKind::ChatDoesNotExist => 404,
    }
}
