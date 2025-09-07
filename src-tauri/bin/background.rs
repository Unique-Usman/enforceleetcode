use my_tauri_app::enforceleetcode::{fetch_leetcode_submissions, shutdown_system};

fn main() {
    if !fetch_leetcode_submissions() {
        println!("No submission today. Shutting down...");
        shutdown_system().unwrap();
    } else {
        println!("Submission found");
    }
}
