use clap::{load_yaml, App};
use crc32fix::*;
use std::fs;
use std::process;

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let inputfile = matches.value_of("inputfile").unwrap();
    let outputfile = matches.value_of("output").unwrap_or("output.png");

    let mut data = fs::read(inputfile).unwrap_or_else(|err| {
        eprintln!("Problem reading input file: {}", err);
        process::exit(1);
    });

    let mut crcdata = CrcData::from_data(&data).unwrap_or_else(|err| {
        eprintln!("Failed to parse the file: {}", err);
        process::exit(1);
    });

    match crcdata.try_fix() {
        Ok(_) => println!("Found! width: {} height: {}", crcdata.width, crcdata.height),
        Err(_) => eprintln!("Not found!"),
    }

    replace_nbytes(&mut data, 12, &crcdata.as_bytes());

    save_file(outputfile, &data).unwrap_or_else(|err| {
        eprintln!("Problem saving output file: {}", err);
        process::exit(1);
    });
}
