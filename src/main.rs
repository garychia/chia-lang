mod chia;
mod common;

use std::fs::File;
use std::io::Read;
use std::{process::exit, vec::Vec};

use chia::lexer::Lexer;

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

fn parse_args() -> Setting {
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

fn read_files(setting: &Setting) -> Result<Vec<String>, String> {
    let mut src_code_strs = Vec::new();
    for file_name in &setting.input_files {
        match File::open(file_name) {
            Ok(mut f) => {
                let mut content = String::new();
                match f.read_to_string(&mut content) {
                    Ok(_) => src_code_strs.push(content),
                    Err(err) => {
                        return Err(format!(
                            "Unable to read the file: {}\nReason: {}",
                            file_name,
                            err.to_string()
                        ));
                    }
                }
            }
            Err(err) => {
                return Err(format!(
                    "Unable to open the file: {}\nReason: {}",
                    file_name,
                    err.to_string()
                ));
            }
        }
    }
    Ok(src_code_strs)
}

fn process_src_code(setting: &Setting, src_contents: Vec<String>) {
    for content in &src_contents {
        let mut lexer = Lexer::new(content);
        while let Some((token, range)) = lexer.next_token() {
            if setting.verbose {
                println!("Token: {}\nPosition: {}", token.to_string(), range.to_string());
            }
        }
    }
}

fn main() {
    print_info();

    let setting = parse_args();

    if setting.input_files.is_empty() {
        print_usage();
        exit(1);
    }
    match read_files(&setting) {
        Ok(src_contents) => process_src_code(&setting, src_contents),
        Err(description) => {
            println!("{}", description);
            exit(1);
        }
    }
}
