use serde::{Deserialize, Serialize};

// take in request - ignore parameters so far
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    #[serde(flatten)]
    pub method: Method,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<StringOrNumber>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrNumber {
    String(String),
    Number(i32),
}

// different methods that the language server will accept
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "method")]
pub enum Method {
    #[serde(rename = "hello")]
    Hello { params: HelloParams },

    #[serde(other)]
    NotSupported,
}

// accepts {"jsonrpc": "2.0", "method": "hello", "params": {"hello": "string" }}
// or      {"jsonrpc": "2.0", "method": "hello", "params": ["string"]}
#[derive(Debug, Serialize, Deserialize)]
pub struct HelloParams {
    pub hello: String,
}

impl ToString for Method {
    fn to_string(&self) -> String {
        let a = match self {
            Method::Hello { params: _ } => "hello",
            Method::NotSupported => "not supported",
        };

        return a.to_string();
    }
}
