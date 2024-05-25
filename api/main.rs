use std::env;
use vercel_runtime::{bundled_api, run, Body, Error, Request, Response};

// #[derive(Deserialize)]
// struct PostData {
//     title: String,
//     content: String,
// }

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    run(handler).await
}


#[bundled_api]
pub async fn handler(req: Request) -> Result<Response<Body>, Error> {}