#![allow(non_snake_case)]

use thirtyfour::prelude::*;
use tokio;
pub struct Scraper{
    pub host: String,
    pub port: String,
    pub driver: Option<WebDriver>,
}
impl Scraper{
    pub async fn newDriver(&mut self) -> Result<(), Box<dyn std::error::Error>>{
        let url = format!("http://{}:{}", self.host, self.port);
        let newDriver = WebDriver::new(&url, DesiredCapabilities::chrome()).await?;
        self.driver = Some(newDriver);
        Ok(())
    }

    pub async fn getSomething(&self, url: &String) -> Result<String ,Box<dyn std::error::Error>>{
        let driver = match &self.driver{
            Some(driver) => driver,
            None => panic!("self.driver is None")
        };
        driver.get(url).await?;
        let title = driver.title().await?;
        Ok(title)
    }
}
/*pub async fn test() -> WebDriverResult<()> {
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
*/