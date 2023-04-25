# Slowly building LSP compliant Language Server
Follow code in examples to build the server from nothing to functional with [**serde**](https://serde.rs/), [**tokio**](https://tokio.rs/) and [**warp**](https://github.com/seanmonstar/warp). If at any point you don't understand something about these libraries, the links should be helpful.

Here are the steps we will break it down into:
1. Create simple HTTP server
2. Turn it into minimal JSON RPC compliant server
3. Improve typing and fletch out the details
4. Turn it into minimal LSP compliant server
    - lexer + parser - defined grammar and created AST	
        - can I user parser combinator like nom instead?
    - virtual file system - to have code in memory, IO is slow
    - individual LSP features
5. Improve typing and expand basic functionality
6. Build a VSCode extension and test it


You will need following dependencies (versions may vary):
```
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0.95"
tokio = {version="1.27.0", features=["full"]}
warp = "0.3.4"
```

## 1. HTTP compliant server (DONE)
First file just creates a HTTP server with Warp. Nothing special, 19 lines of code, including comments. 


Run it yourself, or use the finished example code:
```
cargo run --example 01_http_server
```

Test it by going to http://localhost:3030 or by running
```
curl http://localhost:3030
```
Expected result:
```
<h1>Hello, Warp!</h1>
```

## 2. JSON-RPC minimal server (DONE?)
We need to be compliant with [JSON-RPC 2.0 Spec](https://www.jsonrpc.org/specification). On top of follwing HTTP, we have 2 basic objects:
- request object
     - jsonrpc: "2.0" (`String`, always the same)
     - medthod: `String`
     - params: we will define params later, not needed for minimal server
     - id: `Option<StringOrNumber>` - will handle with `#[serde(untagged)]` ([serde enum representation](https://serde.rs/enum-representations.html))
- response object
     - jsonrpc: "2.0" - same as above
     - result: `Option<String>`
     - error: we will define error later, not needed for minimal server
         - what could possibly go wrong? 
     - id: `Option<StringOrNumber>` - same as above

On top of that we'll'make a new Warp route and return the name of the method that was called.

Run it yourself, or use the finished example code:
```
cargo run --example 02_json_rpc_server
```
Test it with: 
```
curl http://localhost:3030/json_rpc --data '{"jsonrpc": "2.0", "method": "hi", "params": {"hello": "world"},"id": "42"}' --header "Content-Type: application/json" 
```
Expected result:
```
{"jsonrpc":"2.0","result":"You requested method called 'hi'","id":"42"}
```