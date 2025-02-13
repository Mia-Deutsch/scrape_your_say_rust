use thirtyfour::prelude::*;
use tokio;
pub async fn test() -> WebDriverResult<()> {
    //let selenium_host = String::from("localhost");
    //let selenium_port = String::from("4444");
    let selenium_host = std::env::var("SELENIUM_HOST").unwrap();
    let selenium_port = std::env::var("SELENIUM_PORT").unwrap();
    let url = format!("http://{}:{}", selenium_host, selenium_port);

    let driver = WebDriver::new(&url, DesiredCapabilities::chrome()).await?;

    driver.get("https://www.rust-lang.org").await?;

    println!("{}", driver.title().await?);

    driver.quit().await?;
    Ok(())
}