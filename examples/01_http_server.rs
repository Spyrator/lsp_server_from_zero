use warp::Filter;

#[tokio::main]
async fn main() {
    let index = warp::path::end()
        .and(warp::get()) // GET so that we can test it with browser
        .map(|| {
            let body = "<h1>Hello, Warp!</h1>";

            return warp::http::Response::builder()
                .header("content-type", "text/html; charset=utf-8".to_string())
                .status(200)
                .body(body);
        });

    // define paths ahead, we will expand them later
    let paths = index;
    warp::serve(paths).run(([0, 0, 0, 0], 3030)).await;
}
