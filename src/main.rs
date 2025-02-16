use BusinessLogic;
//use DataUtils;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut webScraper = BusinessLogic::Scraper {
        host: std::env::var("SELENIUM_HOST").unwrap(),
        port: std::env::var("SELENIUM_PORT").unwrap(),
        driver: None
    };
    webScraper.newDriver().await?;
    println!("{}", webScraper.getSomething(&String::from("https://www.rust-lang.org")).await?);
    Ok(())
}
