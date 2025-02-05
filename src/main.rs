use std::error::Error;
use thirtyfour::prelude::*;
use tokio;
//use BusinessLogic;
//use DataUtils;
#[tokio::main]
async fn main() -> WebDriverResult<()> {
    //let selenium_host = String::from("localhost");
    //let selenium_port = String::from("4444");
    let selenium_host = std::env::var("SELENIUM_HOST").unwrap();
    let selenium_port = std::env::var("SELENIUM_PORT").unwrap();
    let url = format!("http://{}:{}", selenium_host, selenium_port);

    // Connect to Selenium
    let driver = WebDriver::new(&url, DesiredCapabilities::chrome()).await?;

    // Interact with the browser
    driver.get("https://www.rust-lang.org").await?;
    println!("Page title: {}", driver.title().await?);

    driver.quit().await?;
    Ok(())
}
