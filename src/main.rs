use bytes::*;
use futures::TryStreamExt;
//use std::convert::Infallible;
use uuid::Uuid;
use warp::{
//    http::StatusCode,
    multipart::{FormData, Part},
    Filter, Rejection, Reply,
};
extern crate pretty_env_logger;
#[macro_use] extern crate log;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "trace, info, error, cargo, warp, run");
    pretty_env_logger::init();

    let upload_route = warp::path("upload")
    .and(warp::post())
    .and(warp::multipart::form()/*.max_length(5_000_000)*/)
    .and_then(upload);

    let info = warp::path("info")
    .map(|| "Server is healthy!");

    let folder = warp::fs::dir("./html/");

    //let auth = warp::header("authorization")
    //.map(|token: String| {
        // something with token
    //})
    //.or(default_auth)
    //.unify();


    let routes = info.or(folder).or(upload_route);
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn upload(form: FormData) -> Result<impl Reply, Rejection> {
    let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
        eprintln!("form error: {}", e);
        warp::reject::reject()
    })?;

    info!("About to iterate over parts {}", parts.len());
    warn!("o_O");
    error!("much error");
    for p in parts {
        info!("Iterating over parts {} {:?}", p.name(), p.content_type());
        let content_type = p.content_type();
        if p.name() == "file" || content_type == Some("image/png") {
            
            let file_ending;
            match content_type {
                Some(file_type) => match file_type {
                    "application/pdf" => {
                        file_ending = "pdf";
                    }
                    "image/png" => {
                        file_ending = "png";
                    }
                    "image/apng" => {
                        file_ending = "png";
                    }
                    v => {
                        eprintln!("invalid file type found: {}", v);
                        return Err(warp::reject::reject());
                    }
                },
                None => {
                    eprintln!("file type could not be determined");
                    return Err(warp::reject::reject());
                }
            }

            let value = p
                .stream()
                .try_fold(Vec::new(), |mut vec, data| {
                    vec.put(data);
                    async move { Ok(vec) }
                })
                .await
                .map_err(|e| {
                    eprintln!("reading file error: {}", e);
                    warp::reject::reject()
                })?;

            let file_name = format!("./files/{}.{}", Uuid::new_v4().to_string(), file_ending);
            tokio::fs::write(&file_name, value).await.map_err(|e| {
                eprint!("error writing file: {}", e);
                warp::reject::reject()
            })?;
            println!("created file: {}", file_name);
        }
    }

    Ok("success")
}