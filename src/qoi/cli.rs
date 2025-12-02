use clap::{Command, arg, command, value_parser};
use std::{fs, io::{Stdout, stdout}, path::{Path, PathBuf}};

pub fn cli() {
    let matches = command!()
        
        .subcommand(
            Command::new("encode")
                .about("encodes an image according to the QOI specification")
                .arg(
                    arg!(-i --input <FILE> "input file, from which  to read the data")
                        .required(true)
                        .value_parser(value_parser!(PathBuf)),
                ).arg(arg!(-o --output <FILE> "output file, to which the output is written, defaulted to stdout")
                        .required(false)
                        .value_parser(value_parser!(PathBuf)))
        )
        .subcommand(
            Command::new("decode")
                .about("decodes an image encoded according to the QOI specification")
                .arg(
                    arg!(-i --input <FILE> "input file, from which  to read the data")
                        .required(true)
                        .value_parser(value_parser!(PathBuf)),
                ).arg(arg!(-o --output <FILE> "output file, to which the output is written, defaulted to stdout")
                        .required(false)
                        .value_parser(value_parser!(PathBuf)))

        )
        .get_matches();
    match matches.subcommand(){
        Some(("encode", sub_m)) => {
            let input: &PathBuf = sub_m.get_one("input").unwrap();
            if let Some(output) = sub_m.get_one::<PathBuf>("output"){
                let buffer = fs::read(input).unwrap_or_else(|_| panic!("Error opening the input file: {:?}", input));
            }
            
        }
        Some(("decode", sub_m)) => {
            let input: &PathBuf = sub_m.get_one("input").unwrap();
            let output: &PathBuf = sub_m.get_one("output").unwrap();
            println!("Encoding {:?} in {:?}", input, output);


        }
        _ => {
            eprintln!("Command not found, use the --help flag to understand how the command works")
        }
    }
}
