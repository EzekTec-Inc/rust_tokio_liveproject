// This application showcases working with async (tokio) in rust
use hyper::body::HttpBody as _;
use hyper::Client;
use tokio::io::{self, AsyncWriteExt as _};

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    // pretty_env_logger::init();

    let uri = "http://httpbin.org/ip"; // parse an 'http::Uri' ...
                                       //
                                       // HTTPS requires picking a TLS implementation, so give a better
                                       // warning if user is trying to request an 'https' URL.
    let url = uri.parse::<hyper::Uri>().unwrap();
    if url.scheme_str() != Some("http") {
        println!("This works only with 'http' URLs.");
        return Ok(());
    }

    fetch_url(url).await
}

async fn fetch_url(url: hyper::Uri) -> Result<()> {
    let client = Client::new();
    let mut res = client.get(url).await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    // Stream the body, writing each chunk to stdout as we get it
    // (instead of bufferingAnd printing at the end).
    while let Some(next) = res.data().await {
        let chunk = next?;
        io::stdout().write_all(&chunk).await?;
    }

    println!("\n\nDone!");

    Ok(())
}
