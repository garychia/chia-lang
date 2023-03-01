mod chia;
mod common;

use std::fs::File;
use std::io::Read;
use std::{process::exit, vec::Vec};

use chia::lexer::Lexer;
use common::reserved::ReservedToken;
use common::token::Token;

const VERSION: (u32, u32, u32) = (0, 0, 1);

const HELP_INFO: &str = "Flags:\n-v, --verbose: Verbose Mode";

struct Setting {
    verbose: bool,
    input_files: Vec<String>,
}

fn print_info() {
    println!(
        "Chia Compiler -- Version: {}.{}.{}",
        VERSION.0, VERSION.1, VERSION.2
    );
}

fn print_usage() {
    println!(
        "Usage: {} <flags> <input files>",
        std::env::args().into_iter().next().unwrap()
    );
    println!("{}", HELP_INFO);
}

fn parse_cmd_args() -> Setting {
    let mut verbose = false;
    let mut input_files = Vec::new();
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "-v" | "--verbose" => verbose = true,
            _ => input_files.push(arg),
        }
    }
    Setting {
        verbose: verbose,
        input_files,
    }
}

fn read_files(setting: &Setting) -> Result<Vec<String>, ()> {
    let mut src_code_strs = Vec::new();
    for file_name in &setting.input_files {
        match File::open(file_name) {
            Ok(mut f) => {
                let mut content = String::new();
                match f.read_to_string(&mut content) {
                    Ok(_) => src_code_strs.push(content),
                    Err(err) => {
                        println!(
                            "Unable to read the file: {}\nReason: {}",
                            file_name,
                            err.to_string()
                        );
                        return Err(());
                    }
                }
            }
            Err(err) => {
                println!(
                    "Unable to open the file: {}\nReason: {}",
                    file_name,
                    err.to_string()
                );
                return Err(());
            }
        }
    }
    Ok(src_code_strs)
}

fn process_src_code(src_contents: Vec<String>) {
    for content in &src_contents {
        let mut lexer = Lexer::new(content);
        while let Some((token, _)) = lexer.next_token() {
            println!("{}", token.to_string());
        }
    }
}

fn main() {
    print_info();

    let setting = parse_cmd_args();

    if setting.input_files.is_empty() {
        print_usage();
        exit(1);
    }
    if let Ok(src_contents) = read_files(&setting) {
        process_src_code(src_contents);
    } else {
        exit(1);
    }
}
