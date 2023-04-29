use improve_typing::{
    errors::{ErrorCodes, JsonRpcError},
    request::{JsonRpcRequest, Method},
    response::JsonRpcResponse,
};
use warp::Filter;

use crate::improve_typing::response::JsonRpcResult;
mod improve_typing;

#[tokio::main]
async fn main() {
    // basic endopoint
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
            let result = match &req.method {
                Method::NotSupported => Err(JsonRpcError::new(
                    ErrorCodes::MethodNotFound,
                    Some("The method you called is not supported".to_owned()),
                )),
                _ => Ok(format!(
                    "You requested method called '{}'",
                    &req.method.to_string()
                )),
            };

            let result = JsonRpcResult(result);

            let response = JsonRpcResponse {
                jsonrpc: "2.0".to_owned(),
                result,
                id: req.id,
            };

            // println!("Response: {:#?}", &response);
            // let json = serde_json::to_string(&response).unwrap();
            // println!("Serialized: {:#?}", &json);
            // let res2: JsonRpcResponse = serde_json::from_str(&json).unwrap();
            // println!("Deserialized: {:#?}", &res2);

            return warp::http::Response::builder()
                .header("Content-Type", "application/json; charset=utf-8")
                .status(200)
                .body(serde_json::to_string(&response).unwrap());
        });

    // update paths so they are both included
    let paths = index.or(json_rpc);
    warp::serve(paths).run(([0, 0, 0, 0], 3030)).await;
}
