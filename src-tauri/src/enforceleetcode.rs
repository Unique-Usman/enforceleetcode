use serde_json::json;
use tauri_plugin_http::reqwest;
use tauri_plugin_http::reqwest::header::{HeaderMap, HeaderValue, USER_AGENT}

#[tauri::command]
pub fn enforceleetcode() {}

pub async fn fetch_leetcode_submissions(username: String, date_str: &str) {
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
}
