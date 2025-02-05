use BusinessLogic::test;
//use DataUtils;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    test().await?;
    Ok(())
}
