use std::process::{Command, Stdio};
use pop_launcher::*;

fn main() {
    let query = Request::Search("tetsed".to_string());
    search(query);
    println!("{:?}", response);
}

fn search(query: Request) {
    match query {
        Request::Activate(_) => todo!(),
        Request::ActivateContext { id, context } => todo!(),
        Request::Complete(_) => todo!(),
        Request::Context(_) => todo!(),
        Request::Exit => todo!(),
        Request::Interrupt => todo!(),
        Request::Quit(_) => todo!(),
        Request::Search(_) => todo!(),
    }
}

fn check_spelling(query: &str) -> Vec<String> {
    let echo: std::process::ChildStdout = Command::new("echo")
        .arg(query)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute echo command")
        .stdout
        .expect("Failed to capture echo command output");

    let aspell: std::process::ChildStdout = Command::new("aspell")
        .arg("-a")
        .stdin(Stdio::from(echo))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute aspell command")
        .stdout
        .expect("Failed to capture aspell command output");

    let tail: std::process::Output = Command::new("tail")
        .args(["-n","+2"])
        .stdin(Stdio::from(aspell))
        .output()
        .expect("Failed to execute tail command");

    let aspell_output: String = String::from_utf8_lossy(&tail.stdout)
        .to_string();

    if aspell_output.contains("*") {
        return vec!["Correct spelling".to_string()];
    }
    if aspell_output.contains("#") {
        return vec!["No match found".to_string()];
    }
    let start_index: usize = aspell_output
        .find(":")
        .unwrap_or(0);

    return aspell_output[start_index + 1..]
        .split(", ")
        .map(|s| s.trim().to_string())
        .collect()
}