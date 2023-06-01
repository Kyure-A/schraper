use thirtyfour::prelude::*;
use tokio;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    dotenvy::dotenv()?;
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("--enable-automation")?;
    
    let driver: WebDriver = WebDriver::new("http://localhost:4444", caps).await?;
    driver.goto("https://classroom.google.com").await?;

    let google_account_id: String = dotenvy::var("id").unwrap();
    let google_account_password: String = dotenvy::var("pass").unwrap();
    let class_list: WebElement = driver.find(By::XPath("//ol[@jsname='bN97Pc']")).await?;

    // for とかで囲んで全要素についてやる・差分の確認とか
    let classrooms = class_list.find_all(By::XPath("//li[@jsmodel='hCpsVc bYzLLb AKq4rd']")).await?;
    
    // test
    for classroom in classrooms {
	let _url = classroom.find(By::ClassName("onkcGd ZmqAt Vx8Sxd"));
	println!("{}", classroom);
    }
    
    driver.quit().await?;

    Ok(())
}
