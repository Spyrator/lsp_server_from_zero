use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcError {
    code: i32,
    message: String,
    data: Option<String>, //string | number | boolean | array | object | null;
}
impl JsonRpcError {
    pub fn new(error_type: ErrorCodes, data: Option<String>) -> Self {
        let (code, message) = match error_type {
            ErrorCodes::UnknownErrorCode => (-32001, "UnknownErrorCode".to_owned()),
            ErrorCodes::ServerNotInitialized => (-32002, "ServerNotInitialized".to_owned()),
            ErrorCodes::ParseError => (-32700, "ParseError".to_owned()),
            ErrorCodes::InvalidRequest => (-32600, "InvalidRequest".to_owned()),
            ErrorCodes::MethodNotFound => (-32601, "MethodNotFound".to_owned()),
            ErrorCodes::InvalidParams => (-32602, "InvalidParams".to_owned()),
            ErrorCodes::InternalError => (-32603, "InternalError".to_owned()),
            ErrorCodes::RequestCancelled => (-32800, "RequestCancelled".to_owned()),
            ErrorCodes::ContentModified => (-32801, "ContentModified".to_owned()),
            ErrorCodes::ServerCancelled => (-32802, "ServerCancelled".to_owned()),
            ErrorCodes::RequestFailed => (-32803, "RequestFailed".to_owned()),
        };

        return Self {
            code,
            message,
            data,
        };
    }
}

/**
 * LSP reserved error codes => `-32899 .. -32800`
 * JSON-RPC reserved error codes => `-32099 .. -32000`.
    No LSP error codes should be defined between the start and end range.
    For backwards compatibility the `ServerNotInitialized` and the `UnknownErrorCode` are left in the range
*/
pub enum ErrorCodes {
    // Error code indicating that a server received a notification or request before the server has received the `initialize` request.
    UnknownErrorCode,
    ServerNotInitialized,

    // Defined by JSON-RPC
    ParseError,
    InvalidRequest,
    MethodNotFound,
    InvalidParams,
    InternalError,

    // The client has canceled a request and a server as detected the cancel.
    RequestCancelled,

    /**
     * The server detected that the content of a document got
     * modified outside normal conditions. A server should
     * NOT send this error code if it detects a content change
     * in it unprocessed messages. The result even computed
     * on an older state might still be useful for the client.
     *
     * If a client decides that a result is not of any use anymore
     * the client should cancel the request.
     */
    ContentModified,
    /**
     * The server cancelled the request. This error code should
     * only be used for requests that explicitly support being
     * server cancellable.
     */
    ServerCancelled,
    /**
     * A request failed but it was syntactically correct, e.g the
     * method name was known and the parameters were valid. The error
     * message should contain human readable information about why
     * the request failed.
     */
    RequestFailed,
}
