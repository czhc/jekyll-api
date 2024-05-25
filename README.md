# Jekyll API

Simple API server hosted serverlessly to allow publishing blog posts from a Slack bot


## Dependencies
* Rust 1.78 [[guide](https://doc.rust-lang.org/cargo/getting-started/installation.html)]


## How to Run

1. Run `vercel dev` locally. The API is now available at `http://localhost:3000`
2. Test with 
    ```sh
    curl http://localhost:3000/api/ping
    ```
3. Deploy with vercel

    ```sh
    vercel # or `vercel --prod` for production
    ```
