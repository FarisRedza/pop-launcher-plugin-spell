use std::{io::{self, Read}, process::{Command, Stdio}};
use serde_json::{Deserializer, Serializer};
use pop_launcher::*;

fn main() {
    let mut input = String::new();
    println!("Enter JSON input:");
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read line");
    
    let request: Request = match serde_json::from_str(&input) {
        Ok(req) => req,
        Err(err) => {
            panic!("Failed to parse JSON: {}", err);
        }
    };

    match request {
        Request::Activate(index) => {
            println!("Activate with index: {}", index);
        }
        Request::Search(query) => {
            let response = search(pop_launcher::Request::Search(query));
            println!("Search query: {:?}", response);
        }
        _ => ()
    }

}

fn search(query: Request) {
    match query {
        Request::Search(s) => {
            let search_query = s;
            let checked_query = check_spelling(&search_query);
            let response = PluginResponse::Append(PluginSearchResult {
                id: 0,
                name: checked_query[0].clone(),
                description: search_query,
                icon: None,
                keywords: None,
                exec: None,
                window: None,
            });
            println!("{:?}", response);
        }
        _ => todo!()
    };
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