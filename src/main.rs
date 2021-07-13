use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let info = warp::path("info")
    .map(|| "Server is healthy!");

    let folder = warp::fs::dir("src/html/");

    let routes = info.or(folder);
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}