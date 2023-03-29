use hyper::{Client, body::HttpBody};
use hyper_tls::HttpsConnector;
use tokio::io::{stdout, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let uri = "https://httpbin.org/ip".parse()?;

    let mut response = client.get(uri).await?;

    println!("Response {:#?}", response);

    while let Some(chunk) = response.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }

    Ok(())
}
