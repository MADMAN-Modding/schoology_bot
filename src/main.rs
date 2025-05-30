#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let _ = get_request().await.unwrap();
}

async fn get_request() -> Result<(), reqwest::Error> {
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;

    println!("body = {body:?}");

    Ok(())
}
