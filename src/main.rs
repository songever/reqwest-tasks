use anyhow::Result;
use serde::{Deserialize, Serialize};
#[tokio::main]
async fn main() {
    if let Err(e) = task8_mixed_upload().await {
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
// 任务 3: 发送一个异步 GET 请求并处理 JSON
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
// 任务 4: 发送一个 POST 请求并处理 JSON 响应
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
// 任务 5: 发送自定义请求头
async fn task5_custom_headers() -> Result<()> {
    let response_text = reqwest::Client::new()
        .get("https://httpbin.org/get")
        .header("X-My-Custom-Header", "rust-is-awesome")
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    
    println!("{:#?}", response_text["headers"]);
    Ok(())
}

async fn task6_handle_error1() -> Result<()> {
    let client = reqwest::Client::new();
    let response = client.get("https://httpbin.org/status/404").send().await?;
    let status = response.status();
    if !status.is_success() {
        return Err(anyhow::anyhow!("Request failed with status code: {}", status));
    }
    Ok(())
}

async fn task6_handle_error2() -> Result<()> {
    let client = reqwest::Client::new();
    let response = client.get("https://httpbin.org/status/404").send().await?;
    let status = response.error_for_status()?;
    println!("Request succeeded with status code: {}", status.status());
    Ok(())
}

async fn task7_file_upload() -> Result<()> {
    let client = reqwest::Client::new();
    let form = reqwest::multipart::Form::new();
    let file = reqwest::multipart::Part::file("test_file.txt").await?;

    let response = client.post("https://httpbin.org/post")
        .multipart(form.part("files".to_string(), file))
        .send()
        .await?;

    println!("{}", response.text().await?);
    Ok(())
}

async fn task8_mixed_upload() -> Result<()> {
    #[derive(Debug, Serialize)]
    struct UserMetadata {
        name: String,
        email: String,
    }

    let user_metadata = UserMetadata {
        name: "songever".to_string(),
        email: "1021940593@qq.com".to_string(),
    };

    let client = reqwest::Client::new();
    let form = reqwest::multipart::Form::new();

    let file =  reqwest::multipart::Part::file("test_file.txt").await?;
    let user_metadata_json = serde_json::to_string(&user_metadata)?;
    let json_part = reqwest::multipart::Part::text(user_metadata_json).mime_str("application/json")?;
    let form = form.part("files", file)
        .part("json", json_part);

    let response = client.post("https://httpbin.org/post")
        .multipart(form)
        .send()
        .await?;

    println!("{}", response.text().await?);
    Ok(())
}
