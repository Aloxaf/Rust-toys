#[macro_use] extern crate clap;
extern crate crc32fix;
extern crate png;

use std::fs;
use std::process;
use clap::App;
use crc32fix::*;


fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let inputfile = matches.value_of("inputfile").unwrap();
    let outputfile = matches.value_of("output").unwrap_or("output.png");

    let mut data = fs::read(inputfile).unwrap_or_else(|err| {
        eprintln!("Problem reading input file: {}", err);
        process::exit(1);
    });

    let mut crcdata = CrcData {
        type_str: [0, 0, 0, 0],
        width: 0,
        height: 0,
        bits: 0,
        color_type: 0,
        compr_method: 0,
        filter_method: 0,
        interlace_method: 0,
        crc_val: 0,
    };

    // println!("input: {}\noutput: {}", inputfile, outputfile);

    get_crcdata(&data, &mut crcdata);

    match crack_crc(&mut crcdata) {
        Ok(_) => println!("Found! width: {} height: {}", crcdata.width, crcdata.height),
        Err(_) => eprintln!("Not found!"),
    }

    replace_nbytes(&mut data, 12, &crcdata.as_bytes());

    save_file(outputfile, &data).unwrap_or_else(|err| {
        eprintln!("Problem saving output file: {}", err);
        process::exit(1);
    });

    // println!("{} {}",  crcdata.width, crcdata.height);
}
