use std::env;
use std::fs;
use std::str::FromStr;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;


fn main() {

    let args: Vec<String> = env::args().collect();

    let mut template = fs::read_to_string("src/bin/template.rs").unwrap();

    if args.len() < 2 {
        panic!("Missing day argument");
    }

    let day = u32::from_str(&args[1]).unwrap();
    let day = format!("{:02}", day);

    template.replace("{DAY}", &day);

    let binary_path = format!("src/bin/{}.rs", day);
    if !Path::new(&binary_path).exists() {
        fs::write(&binary_path, template).unwrap();
    }

    let example_file_path = format!("examples/{}.txt", day);
    if !Path::new(&example_file_path).exists() {
        fs::write(&example_file_path, "").unwrap();
    }

    let input_file_path = format!("inputs/{}.txt", day);

    let output = Command::new("aoc").args(["download", "--day", &day, "--input-only", "--input-file", &input_file_path]).output().unwrap();
    println!("{:?}", &output);
}