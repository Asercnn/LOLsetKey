use crate::http;
use base64::{engine::general_purpose, Engine};
use std::error::Error;
use tauri::command;

// lcu.rs
// 这个文件包含了与 League Client Update (LCU) 相关的功能

#[command]
pub fn init() -> Result<String, String> {
    let output = std::process::Command::new("cmd")
        .args([
            "/C",
            "wmic PROCESS WHERE name='LeagueclientUx.exe' GET commandline",
        ])
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;
    let cmd_line = String::from_utf8_lossy(&output.stdout);

    // println!("Command output: {:?}", cmd_line);


    let temptoken: Option<String> = extract_flag_value(&cmd_line, "--remoting-auth-token");
    let port: Option<String> = extract_flag_value(&cmd_line, "--app-port");

    let token: String = temptoken.ok_or("Token not found")?;
    let port: String = port.ok_or("Port not found")?;

    println!("Token: {:?}", token);
    println!("Port: {:?}", port);

    let credentials = format!("riot:{}", token);
    let encoded = general_purpose::STANDARD.encode(credentials.as_bytes()); // Base64 编码
    let authorization = format!("Basic {}", encoded); // 拼接 "Basic " 前缀

    println!("Authorization header: {}", authorization);
    let url = format!("https://127.0.0.1:{}", port);
    let response_text = http::getinit(&url, &authorization)
        .map_err(|e| format!("HTTP request failed: {}", e))?;  
    println!("Response: {:?}", response_text);

    Ok(response_text)
}

fn extract_flag_value(input: &str, flag: &str) -> Option<String> {
    input
        .split_whitespace() // 按照空白分割成多个字符串
        .map(|s| s.trim_matches('"'))
        .find(|s| s.starts_with(flag))
        .and_then(|s| s.split('=').nth(1))
        .map(|s| s.to_string())
}
