use clap::{Command, arg, command, value_parser};
use std::io::Write;
use std::str::from_utf8;
use std::{fs, path::PathBuf};

use crate::qoi::encode;
use crate::qoi::encoder::bytestream_to_pixelstream;
use crate::qoi::types::{DynamicPixel, Pixel, QoiHeader};

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
            let input: &PathBuf = sub_m.get_one("input").expect("Couldn't open the input file correctly");
            let buffer = fs::read(input).unwrap_or_else(|_| panic!("Error reading the input file: {:?}", input));
            let mut array: [DynamicPixel; 64] = [DynamicPixel::Pixel(Pixel::default());64];
            let bytestream = bytestream_to_pixelstream(&buffer);
            let contents: Vec<u8> = encode(&bytestream.0, &mut array, bytestream.1, bytestream.2, bytestream.3).expect("Error in the encoding");
            if let Some(output) = sub_m.get_one::<PathBuf>("output"){
                fs::write(output, contents).expect("Error writing into the output file") 
            } else {
                std::io::stdout().write_all(&contents).expect("Error writing data into stdout")
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
