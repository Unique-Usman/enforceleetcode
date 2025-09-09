use chrono::{NaiveDate, NaiveDateTime, TimeZone, Utc};
use dirs;
use serde_json::{from_str, json, Deserializer, Serializer, Value};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::process::Output;
use tauri::{AppHandle, Manager};
use tauri_plugin_http::reqwest;
use tauri_plugin_http::reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use tauri_plugin_shell::ShellExt;

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

pub fn get_timestamp(date: &str) -> i64 {
    let date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();

    // Combine with midnight (00:00:00)
    let datetime_midnight: NaiveDateTime = date.and_hms_opt(0, 0, 0).unwrap();

    // Convert to UTC DateTime
    let datetime_utc = Utc.from_utc_datetime(&datetime_midnight);

    // Get Unix timestamp (seconds since 1970-01-01 UTC)
    let timestamp = datetime_utc.timestamp();
    timestamp
}

pub fn shutdown_system(app: &AppHandle) -> Result<tauri_plugin_shell::process::Output, String> {
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

pub fn save_username(username: String, app: &AppHandle) -> Result<String, String> {
    let config_dir = app.path().config_dir().unwrap_or(std::path::PathBuf::new());
    std::fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;

    // Full path to config.json
    let config_file = config_dir.join("config.json");

    // Write username
    std::fs::write(
        &config_file,
        format!(r#"{{ "leetcode_username": "{}" }}"#, username),
    )
    .map_err(|e| e.to_string())?;

    Ok("Username saved!".into())
}

pub fn get_username(app: &AppHandle) -> Result<String, String> {
    // Get OS-standard config directory
    let config_dir = match app.path().config_dir() {
        Ok(dir) => dir,
        Err(_) => return Err("Cannot find config directory".into()),
    };

    let config_file = config_dir.join("config.json");

    // Read file contents
    let content = std::fs::read_to_string(config_file).map_err(|e| e.to_string())?;

    // Parse JSON
    let json: serde_json::Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;

    // Extract username
    let username = json["leetcode_username"]
        .as_str()
        .ok_or("Username not found in config")?;

    Ok(username.to_string())
}

fn create_systemd_install_script() -> Result<(), String> {
    // Path to save the script (e.g., user's temp directory)
    let script_path = std::env::temp_dir().join("install_enforceleetcode.sh");

    // Full path to your Tauri binary
    let exec_dir = std::env::current_exe()
        .map_err(|e| e.to_string())?
        .parent()
        .ok_or("Failed to get binary directory")?
        .to_path_buf();

    // Script content
    let script_content = format!(
        r#"
#!/bin/bash
set -e

SERVICE_PATH=/etc/systemd/system/enforceleetcode.service
TIMER_PATH=/etc/systemd/system/enforceleetcode.timer

echo "[Unit]
Description=EnforceLeetCode Daily Submission Check
After=network.target

[Service]
Type=simple
ExecStart={}/background
Restart=on-failure

[Install]
WantedBy=multi-user.target" | sudo tee $SERVICE_PATH

echo "[Unit]
Description=Run EnforceLeetCode Daily

[Timer]
OnCalendar=*-*-* 23:59:00
Persistent=true

[Install]
WantedBy=timers.target" | sudo tee $TIMER_PATH

sudo systemctl daemon-reload
sudo systemctl enable --now enforceleetcode.timer

echo "Systemd service and timer installed successfully."
"#,
        exec_dir.display()
    );

    // Write the script
    fs::write(&script_path, script_content).map_err(|e| e.to_string())?;

    // Make it executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path)
            .map_err(|e| e.to_string())?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms).map_err(|e| e.to_string())?;
    }

    Ok(())
}

pub fn run_install_script() -> Result<(), String> {
    create_systemd_install_script()?;
    let script_path = std::env::temp_dir().join("install_enforceleetcode.sh");

    Command::new("bash")
        .arg(script_path)
        .status()
        .map_err(|e| e.to_string())?;

    Ok(())
}
