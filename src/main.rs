extern crate clap;

mod instruction;
mod machine;

use std::io;
use std::io::{Read, BufReader};
use std::fs::File;

use instruction::Instruction;

fn main() {
    let matches = parse_args();
    let program = match read_program(&matches) {
        Ok(program) => program,
        Err(e) => {
            println!("!! failed to read program: {}", e);
            return;
        }
    };
    let input = matches.value_of("input").unwrap();
    let input_bytes: Vec<u8> = input.bytes().collect();
    let output = machine::execute_program(&program, &input_bytes);
    print_output(&output);
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
            .default_value("")
            .help("program input"))
        .get_matches()
}

fn read_program(matches: &clap::ArgMatches) -> io::Result<Vec<Instruction>> {
    let filename = matches.value_of("program_file").unwrap();
    let file = try!(File::open(filename));
    let reader = BufReader::new(file);

    let mut program = Vec::new();
    for b in reader.bytes() {
        let byte = try!(b);
        if let Some(instr) = Instruction::from_byte(byte) {
            program.push(instr);
        }
    }
    Ok(program)
}

fn print_output(output: &[u8]) {
    for &byte in output {
        print!("{}", byte as char);
    }
}
