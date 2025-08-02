// http.rs
use reqwest::blocking::ClientBuilder;
use std::time::Duration;

pub fn getinit(ip: &str, token: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("{}{}", ip, "/lol-summoner/v1/current-summoner");
    // 创建一个阻塞客户端
    let client = ClientBuilder::new()
        .danger_accept_invalid_certs(true) // 👈 忽略 TLS 证书验证
        .timeout(Duration::from_secs(5))   // 可选：设置超时5s
        .build()?;

    let resp = client
        .get(&url)
        .header("Authorization", format!("{}", token))
        .send()?;

    if resp.status().is_success() {
        let text = resp.text()?;
        Ok(text)
    } else {
        Err(format!("HTTP error: {}", resp.status()).into())
    }
}
