use marion::browser::Browser;
use tokio;
use webdriverbidi::{Capabilities, CapabilityRequest};

#[tokio::main]
async fn main() {
    let always_match = CapabilityRequest::new();
    let capabilities = Capabilities::new(always_match);

    let mut browser = Browser::new(capabilities, "localhost", 4444);

    browser.start().await.unwrap();

    // sleep for 5 seconds
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    browser.stop().await.unwrap();
}
