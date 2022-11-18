///! Handles the results returned from Vale in Json format.
use ansi_brush::Style;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Results {
    #[serde(rename = "stdin.md")]
    stdin: Option<Vec<Result>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
struct Result {
    action: Action,
    check: String,
    description: String,
    line: usize,
    link: String,
    message: String,
    severity: String,
    span: Vec<usize>,
    r#match: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
struct Action {
    name: String,
    params: Option<Vec<String>>,
}

/// Output the results in a pretty way.
pub fn print_results(path: &str, text: &str, results: Results) {
    if let Some(stdin) = results.stdin {
        println!("{}{}", path.bold().underline(), "".reset());
        println!("");
        println!("{}{}", text.magenta(), "".reset());
        println!("");

        for result in stdin {
            print_result(&result);
        }

        println!("");
    }
}

fn print_result(result: &Result) {
    println!(
        "{}\t{}\t{}",
        result.check.cyan(),
        {
            let severity = &result.severity;
            if severity == "error" {
                severity.red()
            } else {
                severity.italic()
            }
        },
        result.message.reset()
    );
}
