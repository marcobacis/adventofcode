use std::env;
use std::fs;
use std::str::FromStr;
use std::path::Path;
use std::process::Command;


fn main() {
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
    let day = format!("{:02}", day);

    let template = match fs::read_to_string("src/bin/template.rs") {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Couldn´t read the template file!");
            std::process::exit(1);
        }
    };

    let bin_content = template.replace("{DAY}", &day);

    let binary_path = format!("src/bin/{}.rs", day);
    if !Path::new(&binary_path).exists() {
        write_to_file_safe(binary_path, bin_content);
    }

    let example_file_path = format!("examples/{}.txt", day);
    if !Path::new(&example_file_path).exists() {
        write_to_file_safe(example_file_path, "".into());
    }

    let input_file_path = format!("inputs/{}.txt", day);

    match Command::new("aoc").args(["download", "--day", &day, "--input-only", "--input-file", &input_file_path]).output() {
        Ok(_) => println!("Successfully downloaded inputs"),
        Err(_) => {
            eprintln!("Error while writing the inputs file {}", input_file_path);
            std::process::exit(1);
        },
    }
    
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