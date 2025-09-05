use chrono::{NaiveDate, NaiveDateTime, TimeZone, Utc};
use serde_json::{from_str, json, Deserializer, Serializer, Value};
use tauri_plugin_http::reqwest;
use tauri_plugin_http::reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};

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
