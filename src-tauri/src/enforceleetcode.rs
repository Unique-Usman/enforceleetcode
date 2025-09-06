use chrono::{NaiveDate, NaiveDateTime, TimeZone, Utc};
use serde_json::{from_str, json, Deserializer, Serializer, Value};
use std::process::Output;
use tauri::AppHandle;
use tauri_plugin_http::reqwest;
use tauri_plugin_http::reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use tauri_plugin_shell::ShellExt;

struct Calendar {}

#[tauri::command]
pub fn enforceleetcode() {}

pub async fn fetch_leetcode_submissions(
    username: String,
    date_str: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let url = "https://leetcode.com/graphql";

    // Build the GraphQL query string
    let query = format!(
        r#"
        {{
            matchedUser(username: "{}") {{
                userCalendar {{
                    activeYears
                    streak
                    submissionCalendar
                }}
            }}
        }}
        "#,
        username
    );
    let mut headers = HeaderMap::new();
    let payload = json!(
            {"query": query, "variables": {}});

    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3"));

    let res = reqwest::Client::new()
        .post(url)
        .headers(headers)
        .json(&payload)
        .send()
        .await?;

    if res.status().is_success() {
        let data: Value = res.json().await?;
        if let Some(matched_user) = data.get("data").and_then(|d| d.get("matchedUser")) {
            if let Some(calendar_str) = matched_user
                .get("userCalendar")
                .and_then(|c| c.get("submissionCalendar"))
                .and_then(|c| c.as_str())
            {
                let calendar_str: Value = from_str(calendar_str)?;
                if let Some(_) = calendar_str.get(format!("{}", get_timestamp(date_str))) {
                    return Ok(true);
                }
            }
        }
    }
    Ok(false)
}

fn get_timestamp(date: &str) -> i64 {
    let date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();

    // Combine with midnight (00:00:00)
    let datetime_midnight: NaiveDateTime = date.and_hms_opt(0, 0, 0).unwrap();

    // Convert to UTC DateTime
    let datetime_utc = Utc.from_utc_datetime(&datetime_midnight);

    // Get Unix timestamp (seconds since 1970-01-01 UTC)
    let timestamp = datetime_utc.timestamp();
    timestamp
}

fn shutdown_system(app: &AppHandle) -> Result<tauri_plugin_shell::process::Output, String> {
    let shell = app.shell();

    let command = if cfg!(target_os = "linux") {
        shell.command("shutdown").args(["now"])
    } else if cfg!(target_os = "macos") {
        shell
            .command("osascript")
            .args(["-e", "tell app \"System Events\" to shut down"])
    } else if cfg!(target_os = "windows") {
        shell.command("shutdown").args(["/s", "/t", "0"])
    } else {
        return Err("Unsupported OS".into());
    };

    let output = tauri::async_runtime::block_on(async move {
        command.output().await.map_err(|e| e.to_string())
    })?;

    Ok(output)
}
