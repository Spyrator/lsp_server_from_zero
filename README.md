# Building LSP compliant Language Server Step by Step
Code in examples is setup so that each example is a step after previous example.We'll be using with [**serde**](https://serde.rs/), [**tokio**](https://tokio.rs/) and [**warp**](https://github.com/seanmonstar/warp). 

The language of choice for this example is SQL.

Here are the steps we will break it down into:
1. Create simple HTTP server (✅)
2. Turn it into minimal JSON RPC compliant server (✅)
3. Improve typing and accept parameters (⚠️)
4. Turn it into minimal LSP compliant server (🚫)
    - before initialize -> error: -32002
    - initialize, initialized, register/unregister capacity, set/log trace, shutdown, exit
5. Improve typing and expand basic functionality (🚫)
    - lexer + parser - defined grammar and created AST	
        - can I user parser combinator like [nom](https://github.com/rust-bakery/nom) instead? ([guide](https://tfpk.github.io/nominomicon/chapter_1.html))
        - or should I use [ANTLR v4](https://github.com/antlr/grammars-v4/tree/master/sql/postgresql)?
    - virtual file system - to have code in memory, IO is slow
    - individual LSP features
6. Build a VSCode extension and test it (🚫)


You will need following dependencies (versions may vary):
```
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0.95"
tokio = {version="1.27.0", features=["full"]}
warp = "0.3.4"
```

## 1. HTTP compliant server  (✅)
First file just creates a HTTP server with Warp. 


Example code:
```
cargo run --example 01_http_server
```

Test:
```
curl http://localhost:3030
```
Expected result:
```
<h1>Hello, Warp!</h1>
```

## 2. JSON-RPC minimal server  (✅)
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

In new route we return the name of the method that was called.

Example code:
```
cargo run --example 02_json_rpc_server
```
Test: 
```
curl http://localhost:3030/json_rpc --data '{"jsonrpc": "2.0", "method": "hello", "params": {"hello": "world"},"id": "42"}' --header "Content-Type: application/json" 
```
Expected result:
```
{"jsonrpc":"2.0","result":"You requested method called 'hello'","id":"42"}
```


## 3. Improving typing and accept parameters
### Files
New structure with folder named as example name without the number `improve_typing` for `03_improve_typing.rs` example.
### Parameters
Until now we ignored request `parameters`, so let's define them. Let's couple the acceptable parameters with the method name using enums and `#[serde(tag = "method")]`. 

It lets us avoid parsing generic js type `array | object`. We'll match on  accepted methods and use `#[serde(other)]` to reject unaccepted ones (or wrong parameters).

Serde takes care of [positional arguments](https://www.jsonrpc.org/specification#parameter_structures) by default.

### Errors
Defined possible error in errors.rs. These are based on JSON-RPC & LSP reserved codes. For more read [LSP spec - response message](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#responseMessage).

We'll change response type to use `Result` type and implement our own Serialize and Deserialize traits, because specs require the fields to be named `result` and `error`, but `Result` type deserializes to `ok` and `err`.

⚠️Work in Progress⚠️