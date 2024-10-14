use std::io::{self, BufRead, Write};

use pop_launcher::*;
use copypasta::ClipboardProvider;

struct App {
    matches: Option<Vec<String>>
}

impl App {
    fn new() -> Self {
        App { matches: None }
    }

    fn check_spelling(&self, word: &str) -> Vec<String> {
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
            _ => result
                .split(',')
                .map(|word| word.trim().to_string())
                .collect(),
        };
        return result_list;
    }
    
    fn activate(&self, index: usize) {
        if let Some(matches) = &self.matches {
            if !matches.is_empty() {
                let mut ctx: copypasta::x11_clipboard::X11ClipboardContext = copypasta::ClipboardContext::new()
                    .unwrap();
                let selection: &String = &matches[index];
                ctx.set_contents(selection.to_owned())
                    .unwrap();
                let _content: String = ctx
                    .get_contents()
                    .unwrap();
                println!("\"Close\"");
            }
        }
    }

    fn search(&mut self, query: &str) {
        let query = query.split_whitespace().skip(1).collect::<Vec<&str>>().join(" ");
        self.matches = Some(self.check_spelling(&query));

        if let Some(matches) = &self.matches {
            for (index, word) in matches.iter().enumerate() {
                let (name, description) = if matches.len() == 1 {
                    (query.clone(), word.clone())
                } else {
                    (word.clone(), query.clone())
                };
                let response: PluginResponse = PluginResponse::Append( PluginSearchResult {
                    id: index as u32,
                    name: name,
                    description: format!("Spell check: {}", description),
                    ..Default::default()
                });
                println!("{}", serde_json::to_string(&response).unwrap());
            }
        }
        println!("\"Finished\"");
    }
}

fn main() {
    let mut app: App = App::new();
    let stdin: io::Stdin = io::stdin();

    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            let request: serde_json::Value = serde_json::from_str(&line)
                .unwrap_or(serde_json::Value::Null);
            
            if let Some(search_query) = request.get("Search") {
                if let Some(query) = search_query.as_str() {
                    app.search(query);
                }
            } else if let Some(activate_idx) = request.get("Activate") {
                if let Some(index) = activate_idx.as_u64() {
                    app.activate(index as usize);
                }
            }
        }
    }
}
