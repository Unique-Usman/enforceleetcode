use your_crate_name::enforceleetcode::{
    fetch_leetcode_submissions, run_install_script, save_username,
};

fn main() {
    if !fetch_leetcode_submissions() {
        println!("No submission today. Shutting down...");
        shutdown_system().unwrap();
    } else {
        println!("Submission found");
    }
}
