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
struct Config {
    auth_token: String,
    github_token: String,
    github_username: String,
    github_repo: String,
}

impl Config {
    fn from_env() -> Result<Self, Error> {
        dotenv::dotenv().ok();
        let auth_token = env::var("BASIC_AUTH_TOKEN").map_err(|e| Error::from(e.to_string()))?;
        let github_token = env::var("GITHUB_TOKEN").map_err(|e| Error::from(e.to_string()))?;
        let github_username = env::var("GITHUB_USERNAME").map_err(|e| Error::from(e.to_string()))?;
        let github_repo = env::var("GITHUB_REPO").map_err(|e| Error::from(e.to_string()))?;
        Ok(Self {
            auth_token,
            github_token,
            github_username,
            github_repo,
        })
    }
}

async fn authenticate(req: &Request, auth_token: &str) -> Result<(), Response<Body>> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if auth_header.to_str().unwrap_or("") != auth_token {
            return Err(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header("Content-Type", "application/json")
                .body(Body::from(json!({"error": "Unauthorized"}).to_string()))
                .expect("Failed to create response"));
        }
    } else {
        return Err(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("Content-Type", "application/json")
            .body(Body::from(json!({"error": "Unauthorized"}).to_string()))
            .expect("Failed to create response"));
    }
    Ok(())
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let config = Config::from_env()?;
    authenticate(&req, &config.auth_token).await?;

    let timestamp = Utc::now().timestamp().to_string();
    let filename = format!("collections/_bytes/{}.md", timestamp);
    let date = Utc::now().format("%Y-%m-%d %H:%M:%S %z").to_string();

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
