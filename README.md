# Jekyll API

Simple API server hosted serverlessly to allow publishing blog posts from a Slack bot


## Dependencies
* Rust 1.78 [[guide](https://doc.rust-lang.org/cargo/getting-started/installation.html)]


## How to Run

1. Copy `.env.example` to `.env`. Modify the file with the required variables

```sh
cp .env.example .env
```

2. Run `cargo run`. The API is now available at `http://localhost:3000`
