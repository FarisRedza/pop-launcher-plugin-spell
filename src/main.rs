use std::io::{self, BufRead, Write};
use futures_lite::stream::StreamExt;
use pop_launcher::*;
use copypasta::ClipboardProvider;

struct App {
    matches: Option<Vec<String>>,
    descriptions: Option<Vec<String>>
}

impl App {
    fn new() -> Self {
        App {
            matches: None,
            descriptions: None
        }
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
    
    fn activate(&self, index: u32) {
        if let Some(matches) = &self.matches {
            if !matches.is_empty() {
                let mut ctx: copypasta::x11_clipboard::X11ClipboardContext = copypasta::ClipboardContext::new()
                    .unwrap();
                let selection: &String = &matches[index as usize];
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
        let query: String = query
            .split_whitespace()
            .skip(1)
            .collect::<Vec<&str>>()
            .join(" ");

        let result: Vec<String> = self.check_spelling(&query);

        if result[0] == String::from("Correct spelling") || result[0] == String::from("No match found") {
            self.matches = Some(vec![query.clone(); result.len()]);
            self.descriptions = Some(result);
        } else {
            self.matches = Some(result.clone());
            self.descriptions = Some(vec![query.clone(); result.len()]);
        }

        if let (Some(matches), Some(descriptions)) = (&self.matches, &self.descriptions) {
            for (index, word) in matches.iter().enumerate() {
                let response: PluginResponse = PluginResponse::Append( PluginSearchResult {
                    id: index as u32,
                    name: word.to_string(),
                    description: format!("Spell check: {}", descriptions[index]),
                    ..Default::default()
                });
                println!("{}", serde_json::to_string(&response).unwrap());
            }
        }
        println!("\"Finished\"");
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut requests = json_input_stream(async_stdin());
    let mut app: App = App::new();

    while let Some(result) = requests.next().await {
        match result {
            Ok(request) => match request {
                Request::Activate(id) => app.activate(id),
                // Request::ActivateContext { .. } => app.activate_context().await,
                // Request::Context(_) => app.context().await,
                Request::Search(query) => app.search(&query),
                Request::Exit => break,
                _ => (),
            },
            Err(why) => {
                tracing::error!("malformed JSON input: {}", why);
            }
        }
    }
}