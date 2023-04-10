use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let path = req.uri().path();
    let query = req.uri().query();
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({ "path": path,
            "query": match query {
                    None => "none",
                    Some(q) => q,
                }})
            .to_string()
            .into(),
        )?)
}
