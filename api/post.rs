use vercel_runtime::{Body, Error, Request, Response, StatusCode};
use octocrab::Octocrab;
use chrono::Utc;

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
//     let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");
//     let octocrab = Octocrab::builder().personal_token(github_token).build().unwrap();
//     let owner = env::var("GITHUB_USERNAME").expect("GITHUB_USERNAME must be set");
//     let repo = env::var("GITHUB_REPO").expect("GITHUB_REPO must be set");

//     let timestamp = Utc::now().timestamp().to_string();
//     let filename = format!("collections/_bytes/{}.md", timestamp);
//     let date = Utc::now().format("%Y-%m-%d %H:%M:%S %z").to_string();
//     let file_content = format!(
//         "---\n\
//         title: \"{}\"\n\
//         collection: bytes\n\
//         categories: [bytes]\n\
//         layout: bytes\n\
//         date: \"{}\"\n\
//         ---\n\n\
//         {}",
//         post_data.title, date, post_data.content
//     );

//     let commit_message = format!("Add {}", filename);

//     let response = octocrab
//         .repos(owner, repo)
//         .create_file(&filename, &commit_message, &file_content)
//         .branch("main")
//         .send()
//         .await;

//     match response {
//         Ok(_) => Ok(warp::reply::with_status(
//             warp::reply::json(&"Post created successfully"),
//             StatusCode::OK,
//         )),
//         Err(e) => Ok(warp::reply::with_status(
//             warp::reply::json(&format!("Failed to create post: {}", e)),
//             StatusCode::INTERNAL_SERVER_ERROR,
//         )),
//     }
    Ok(Response::builder()
    .status(StatusCode::OK)
    .header("Content-Type", "application/json")
    .body(Body::Text("Post created successfully".into()))?)

}
