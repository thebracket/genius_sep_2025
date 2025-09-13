use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = env::var("URL").unwrap_or_else(|_| "http://127.0.0.1:8080/".to_string());
    let user = env::var("USER_NAME").unwrap_or_else(|_| "Alice".to_string());

    let client = reqwest::Client::builder().build()?;
    let response = client
        .get(&url)
        .header(reqwest::header::COOKIE, format!("User={}", user))
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    println!("Status: {}", status);
    println!("{}", body);

    Ok(())
}
