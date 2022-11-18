use std::{
    fs::File,
    io::BufReader,
    path::PathBuf,
    process::{Command, Stdio},
};

use clap::Parser;
use regex::Regex;

mod results;

use crate::results::print_results;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    json: PathBuf,

    /// Location of a file that contains the paths in our json file that we
    /// should send to Vale. The path can be a regex pattern.
    #[arg(short, long)]
    include: PathBuf,
}

fn main() {
    let args = Args::parse();

    let includes = std::fs::read_to_string(args.include).unwrap();

    let includes = includes
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| Regex::new(line).unwrap())
        .collect::<Vec<_>>();

    let file = File::open(args.json).unwrap();
    let reader = BufReader::new(file);

    let json: serde_json::Value = serde_json::from_reader(reader).unwrap();

    walk_json(json, &includes, String::new());
}

fn walk_json(value: serde_json::Value, includes: &[Regex], path: String) {
    match value {
        serde_json::Value::Array(array) => {
            for (idx, value) in array.into_iter().enumerate() {
                walk_json(value, includes, format!("{}[{}]", path, idx));
            }
        }
        serde_json::Value::Object(object) => {
            for (key, value) in object {
                let path = format!("{}.{}", path, key);

                if includes.iter().any(|include| include.is_match(&path)) {
                    vale(value, &path);
                } else {
                    walk_json(value, includes, path);
                }
            }
        }
        _ => (),
    }
}

/// Run Vale on the given value (if that Value is a Json string).
fn vale(value: serde_json::Value, path: &str) {
    use std::io::{Read, Write};

    match value {
        serde_json::Value::String(text) => {
            let command = Command::new("fish")
                .current_dir("/home/stephenwakely/src/valestuff/")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .args(["-c", "vale --ext .md --output JSON"])
                .spawn()
                .unwrap();

            write!(command.stdin.unwrap(), "{}", text).unwrap();

            let mut output = String::new();
            command.stdout.unwrap().read_to_string(&mut output).unwrap();

            let result = serde_json::from_str(&output).unwrap();
            print_results(path, &text, result);
        }
        _ => (),
    }
}
