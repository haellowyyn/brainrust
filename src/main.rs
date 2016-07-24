extern crate clap;

use std::io;
use std::io::{Read, BufReader};
use std::fs::File;

fn main() {
    let matches = parse_args();
    let program = match read_program(&matches) {
        Ok(program) => program,
        Err(e) => {
            println!("!! failed to read program: {}", e);
            return;
        }
    };
    execute_program(&program);
}

fn parse_args<'a>() -> clap::ArgMatches<'a> {
    clap::App::new("brainrust")
        .author("Jan Teske")
        .version("0.1.0")
        .about("A brainfuck interpreter written in Rust.")
        .arg(clap::Arg::with_name("program_file")
            .value_name("program-file")
            .required(true)
            .help("file containing the program to execute"))
        .arg(clap::Arg::with_name("input")
            .short("i")
            .value_name("input")
            .required(false)
            .help("program input"))
        .get_matches()
}

fn read_program(matches: &clap::ArgMatches) -> io::Result<String> {
    let filename = matches.value_of("program_file").unwrap();
    let file = try!(File::open(filename));
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    try!(reader.read_to_string(&mut buf));
    Ok(buf)
}

fn execute_program(program: &str) {
    // TODO
}
