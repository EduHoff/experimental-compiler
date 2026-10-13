use std::{env, fs::read_to_string};

use experimental_compiler::compiler::lexer;

fn main() {
    let args: Vec<String> = env::args().collect();

    let program_name = match args.first() {
        Some(name) => name,
        None => "unkown program",
    };

    if args.len() != 2 {
        eprintln!("Error: Invalid usage!");
        eprintln!("Usage: {program_name} <file.exl>");
        std::process::exit(1);
    }

    let Some(file_path) = args.get(1) else {
        eprintln!("Error: Missing file argument!");
        std::process::exit(1);
    };

    let contents = match read_to_string(file_path) {
        Ok(txt) => txt,
        Err(err) => {
            eprintln!("Error reading file '{file_path}': {err}");
            std::process::exit(1);
        }
    };

    println!("{}", &contents);

    let token_list = lexer::tokenize(&contents);

    for tk in &token_list {
        println!("{tk:?}\n");
    }

    std::process::exit(0);
}
