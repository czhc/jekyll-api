use vercel_runtime::{Body, Error, Request, Response, StatusCode};
use octocrab::{Octocrab};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, from_slice};
use std::env;

#[derive(Deserialize)]
struct PostData {
    title: String,
    content: String,
}

#[derive(Serialize)]
struct SuccessResponse {
    title: String,
    date: String,
    collection: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    dotenv::dotenv().ok();

    let auth_token = env::var("BASIC_AUTH_TOKEN").expect("BASIC_AUTH_TOKEN must be set");
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");
    let octocrab = Octocrab::builder().personal_token(github_token).build().unwrap();
    let owner = env::var("GITHUB_USERNAME").expect("GITHUB_USERNAME must be set");
    let repo = env::var("GITHUB_REPO").expect("GITHUB_REPO must be set");

    let timestamp = Utc::now().timestamp().to_string();
    let filename = format!("collections/_bytes/{}.md", timestamp);
    let date = Utc::now().format("%Y-%m-%d %H:%M:%S %z").to_string();

    let header = req.headers();

    // Get the authorization header
    if let Some(auth_header) = header.get("Authorization") {
        if auth_header.to_str().unwrap_or("") != auth_token {
            return Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header("Content-Type", "application/json")
                .body(Body::from(json!({"error": "Unauthorized"}).to_string()))?);
        }
    } else {
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("Content-Type", "application/json")
            .body(Body::from(json!({"error": "Unauthorized"}).to_string()))?);
    };

    let body = req.body();
    let post_data: PostData = match from_slice(body) {
        Ok(data) => data,
        Err(_) => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("Invalid request body".into())
                .expect("Failed to create response"))
        }
    };

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
            Ok(_) => {
                let success_response = SuccessResponse {
                    title: post_data.title,
                    date,
                    collection: "bytes".into(),
                };
                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "application/json")
                    .body(Body::Text(serde_json::to_string(&success_response)?))
                    .expect("Failed to create response"))
            }
            Err(e) => {
                let error_response = ErrorResponse {
                    error: e.to_string(),
                };
                Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header("Content-Type", "application/json")
                    .body(Body::Text(serde_json::to_string(&error_response)?))
                    .expect("Failed to create response"))
            }
        }
}
