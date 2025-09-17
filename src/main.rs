use anyhow::Result;
use serde::{Deserialize, Serialize};
#[tokio::main]
async fn main() {
    if let Err(e) = task5_custom_headers().await {
        eprintln!("Error: {:#?}", e);
    }
}

async fn task1_get_request() -> Result<()> {
    let response = reqwest::get("https://httpbin.org/get").await?;
    println!("{:#?}", response);
    Ok(())
}

async fn task2_get_with_params() -> Result<()> {
    // 这次请求 https://httpbin.org/get，但同时附带两个查询参数，例如 name=rust 和 lang=reqwest。
    // 打印出返回的响应体，观察 args 字段是否包含了你发送的参数。
    let response = reqwest::Client::new()
        .get("https://httpbin.org/get")
        .query(&[("name", "rust"), ("lang","reqwest")])
        .send()
        .await?;

    println!("{}", response.text().await?);
    Ok(())
}

#[derive(Debug, Deserialize)]
struct Mystruct {
    slideshow: Slideshow,
}

#[derive(Debug, Deserialize)]
struct Slide {
    title: String,
    #[serde(default)]
    r#type: Option<String>,
    #[serde(default)]
    items: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct Slideshow {
    author: String,
    date: String,
    slides: Vec<Slide>,
    title: String,
}

async fn task3_async_json() -> Result<()> {
    let response = reqwest::get("https://httpbin.org/json").await?;
    let response = response.json::<Mystruct>().await?;
    println!("{:#?}", response);
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
struct Myrequest {
    user: String,
    id: u16,
    name: String,
}

async fn task4_post_form() -> Result<()> {
    let request = Myrequest {
        user: "1021940593@qq.com".to_string(),
        id: 1,
        name: "songever".to_string(),
    };

    let client = reqwest::Client::new();
    let response = client
        .post("https://httpbin.org/post")
        // .form(&request)?
        .json(&request)
        .send()
        .await?;

    println!("{}", response.text().await?);
    Ok(())
}

async fn task5_custom_headers() -> Result<()> {
    let response = reqwest::Client::new()
        .get("https://httpbin.org/get")
        .header("X-My-Custom-Header", "rust-is-awesome")
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    
    println!("{:#?}", response["headers"]);
    Ok(())
}