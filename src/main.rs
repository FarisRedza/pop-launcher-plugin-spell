use std::io::{self, BufRead, Write};

use pop_launcher::*;
use copypasta::ClipboardProvider;

fn main() {
    // Read JSON input from stdin
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Collect input into a string
    let mut input = String::new();
    for line in handle.lines() {
        input.push_str(&line.unwrap());
    }

    // Parse the JSON input
    let json_value: serde_json::Value = serde_json::from_str(&input)
        .expect("Failed to parse JSON");

    // Match on keys and call corresponding functions
    match json_value {
        serde_json::Value::Object(map) => {
            for (key, value) in map {
                match key.as_str() {
                    "Search" => check_spelling(value.to_string()),
                    "Exit" => break,
                    _ => println!("Unhandled key: {}", key),
                }
            }
        },
        _ => println!("Expected a JSON object."),
    }
}

// fn main() {
//     let mut input: String = String::new();

//     println!("Please enter query: ");
//     std::io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read line");

//     check_spelling(input.trim().to_string());
// }

fn check_spelling(word: String) {
    let mut aspell: std::process::Child = std::process::Command::new("aspell")
        .arg("-a")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to start `aspell` command");

    if let Some(stdin) = aspell
        .stdin
        .as_mut() {
            stdin
                .write_all(word.as_bytes())
                .expect("Failed to write to stdin");
        }

    let output: std::process::Output = aspell
        .wait_with_output()
        .expect("Failed to read stdout");

    let stdout: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
    let result: &str = stdout
        .splitn(2, '\n')
        .nth(1)
        .unwrap_or("")
        .trim();

    let result_type: char = result
        .chars()
        .next()
        .unwrap();

    let result: &str = stdout
        .splitn(2, ':')
        .nth(1)
        .unwrap_or("")
        .trim();

    let result_list: Vec<String> = match result_type {
        '*' => vec![String::from("Correct spelling")],
        '#' => vec![String::from("No match found")],
        _ => result.split(',').map(|word| word.trim().to_string()).collect(),
    };

    let mut ctx: copypasta::x11_clipboard::X11ClipboardContext = copypasta::ClipboardContext::new()
        .unwrap();
    let msg: &String = match result_list.len() {
        1 => &word,
        _ => &result_list[0]
    };
    ctx.set_contents(msg.to_owned()).unwrap();
    let _content: String = ctx.get_contents().unwrap();

    for (i, result) in result_list.iter().enumerate() {
        let (name, description) = if result_list.len() == 1 {
            (word.clone(), result.clone())
        } else {
            (result.clone(), word.clone())
        };
    
        let response = PluginResponse::Append(PluginSearchResult {
            id: i as Indice,
            name: name,
            description: format!("Spell check: {}", description),
            ..Default::default()
        });
        println!("{:?}", response);
    }
}
