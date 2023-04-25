use serde::{Deserialize, Serialize};
use warp::Filter;

// take in request - ignore parameters so far
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    #[serde(skip)]
    pub params: Option<String>, // spec: array | object - we define it later

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<StringOrNumber>,
}

// return a response
// - no error so far
// - result is only String, but is valid JSON RPC already
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>, // spec: string | number | boolean | array | object | null, we define it later
    #[serde(skip)]
    pub error: Option<String>, // we define it later

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<StringOrNumber>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrNumber {
    String(String),
    Number(i32),
}

#[tokio::main]
async fn main() {
    // basic server
    let index = warp::path::end()
        .and(warp::get()) // GET so that we can test it with browser
        .map(|| {
            // point to another endpoint
            let body = "<p>Hey, go to /json_rpc to submit data</p>
                <p>data should be in format: ###</p>";

            return warp::http::Response::builder()
                .header("content-type", "text/html; charset=utf-8".to_string())
                .status(200)
                .body(body);
        });

    // rpc endpoint
    let json_rpc = warp::path("json_rpc")
        .and(warp::post()) // must be POST by protocol
        .and(warp::header::exact("Content-Type", "application/json"))
        // `Content-Length` must be specified, but is everywhere automatically I guess, so no need to require it
        .and(warp::body::json())
        .map(|req: JsonRpcRequest| {
            let res = JsonRpcResponse {
                jsonrpc: "2.0".to_owned(),
                result: Some(format!("You requested method called '{}'", &req.method)),
                error: None,
                id: req.id,
            };

            return warp::http::Response::builder()
                .header("Content-Type", "application/json; charset=utf-8")
                .status(200)
                .body(serde_json::to_string(&res).unwrap());
        });

    // update paths so they are both included
    let paths = index.or(json_rpc);
    warp::serve(paths).run(([0, 0, 0, 0], 3030)).await;
}
