use warp::Filter;
use serde::Deserialize;
use octocrab::Octocrab;
use std::env;
use chrono::Utc;
use warp::http::StatusCode;

#[derive(Deserialize)]
struct PostData {
    title: String,
    content: String,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let root_handler = warp::path::end().map(|| warp::reply::html("You shall not pass!\n"));
    let post_handler = warp::path("post")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_post);

    let api = root_handler.or(post_handler);
    println!("Starting server on 0.0.0.0:3030");

    warp::serve(api).run(([0, 0, 0, 0], 3030)).await;
}

async fn handle_post(post_data: PostData) -> Result<impl warp::Reply, warp::Rejection> {
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");
    let octocrab = Octocrab::builder().personal_token(github_token).build().unwrap();
    let owner = env::var("GITHUB_USERNAME").expect("GITHUB_USERNAME must be set");
    let repo = env::var("GITHUB_REPO").expect("GITHUB_REPO must be set");

    let timestamp = Utc::now().timestamp().to_string();
    let filename = format!("collections/_bytes/{}.md", timestamp);
    let date = Utc::now().format("%Y-%m-%d %H:%M:%S %z").to_string();
    let file_content = format!(
        "---\n\
        title: \"{}\"\n\
        collection: bytes\n\
        categories: [bytes]\n\
        layout: bytes\n\
        date: \"{}\"\n\
        ---\n\n\
        {}",
        post_data.title, date, post_data.content
    );

    let commit_message = format!("Add {}", filename);

    let response = octocrab
        .repos(owner, repo)
        .create_file(&filename, &commit_message, &file_content)
        .branch("main")
        .send()
        .await;

    match response {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&"Post created successfully"),
            StatusCode::OK,
        )),
        Err(e) => Ok(warp::reply::with_status(
            warp::reply::json(&format!("Failed to create post: {}", e)),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}
