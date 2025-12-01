
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::str::FromStr;
use std::path::Path;
use std::sync::Arc;
use anyhow::Result;
use reqwest::Url;
use reqwest::cookie::Jar;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Missing day argument");
        std::process::exit(1);
    }

    let day = match u32::from_str(&args[1]) {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Day arugment must be a number");
            std::process::exit(1);
        }
    };
    let day_formatted = format!("{:02}", day);

    let template = match fs::read_to_string("src/bin/template.rs") {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Couldn´t read the template file!");
            std::process::exit(1);
        }
    };

    let bin_content = template.replace("{DAY}", &day_formatted);

    let binary_path = format!("src/bin/{}.rs", day_formatted);
    if !Path::new(&binary_path).exists() {
        write_to_file_safe(binary_path, bin_content);
    }

    let example_file_path = format!("examples/{}.txt", day_formatted);
    if !Path::new(&example_file_path).exists() {
        write_to_file_safe(example_file_path, "".into());
    }


    download_input(day)?;

    Ok(())
}

fn download_input(day: u32) -> Result<()> {

    let session = fs::read_to_string(".session")?;

    let day_formatted = format!("{:02}", day);
    let input_file_path = format!("inputs/{}.txt", day_formatted);
    let url = Url::parse(&format!("https://adventofcode.com/2025/day/{day}/input"))?;

    let cookie = format!("session={}", session.trim());
    let jar = Jar::default();
    jar.add_cookie_str(&cookie, &url);
    let client = reqwest::blocking::Client::builder().cookie_provider(Arc::new(jar)).build()?;

    let mut resp = client.get(url).send()?;
    let mut out = File::create(input_file_path)?;
    io::copy(&mut resp, &mut out)?;
    Ok(())
}

fn write_to_file_safe(binary_path: String, bin_content: String) {
    match fs::write(&binary_path, bin_content) {
        Ok(_) => {},
        Err(_) => {
            eprintln!("Error while writing file {}", binary_path);
            std::process::exit(1);
        },
    }
}