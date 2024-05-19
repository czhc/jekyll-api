use warp::Filter;
use serde::Deserialize;
use octocrab::Octocrab;
use std::env;
use chrono::Utc;
use base64::encode;

#[derive(Deserialize)]
struct PostData {
    title: String,
    content: String,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let api = warp::path("post")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_post);

    warp::serve(api).run(([127, 0, 0, 1], 3030)).await;
}

async fn handle_post(post_data: PostData) -> Result<impl warp::Reply, warp::Rejection> {
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");
    let octocrab = Octocrab::builder().personal_token(github_token).build().unwrap();

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

    let encoded_content = encode(file_content);
    let owner = env::var("GITHUB_USERNAME").expect("GITHUB_USERNAME must be set");
    let repo = env::var("GITHUB_REPO").expect("GITHUB_REPO must be set");

    let response = octocrab
        .repos(owner, repo)
        .create_file(&filename, &encoded_content, &format!("Create new post: {}", post_data.title))
        .branch("main")
        .send()
        .await;


    match response {
        Ok(_) => Ok(warp::reply::json(&"Post created successfully")),
        Err(e) => Ok(warp::reply::json(&format!("Failed to create post: {}", e))),
    }
}
