use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    
    driver.goto("https://classroom.google.com").await?;

    let classList = driver.find(By::XPath("//ol[@jsname='bN97Pc']")).await?;
    
    // for とかで囲んで全要素についてやる・差分の確認とか
    let class = driver.find(By::XPath("//li[@jsmodel='hCpsVc bYzLLb AKq4rd']")).await?;
    
    driver.quit().await?;
}
