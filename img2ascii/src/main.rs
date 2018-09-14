#[macro_use] extern crate clap;
extern crate image;
extern crate img2ascii;

use image::{imageops, GenericImage};
use clap::App;
use std::io::prelude::*;
use std::path::Path;
use std::process;
use std::fs::File;
use img2ascii::image2ascii;

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let inputfile = matches.value_of("filename").unwrap();
    let width: u32  = value_t!(matches.value_of("width"), u32).unwrap_or(0);
    let height: u32 = value_t!(matches.value_of("height"), u32).unwrap_or(0);

    let image = image::open(inputfile).unwrap_or_else(|e| {
        eprintln!("failed to open {}:\n  {}", inputfile, e);
        process::exit(1);
    });

    let (img_w, img_h) = image.dimensions();

    let (width, height) = match (width, height) {
        (0, 0) => (80, 80 * img_h / img_w),
        (0, h) => (h * img_w / img_h, h),
        (w, 0) => (w, w * img_h / img_w),
        (w, h) => (w, h),
    };


    let image = image.resize(width, height, imageops::Nearest).to_luma();
    let output = image2ascii(&image);

    if let Some(outputfile) = matches.value_of("output") {
        if Path::new(outputfile).exists() && matches.occurrences_of("force") == 0 {
            eprintln!("failed to write {}:\n  file already exists!", outputfile);
            process::exit(1);
        }
        let mut f = File::create(outputfile).unwrap_or_else(|e| {
            eprintln!("failed to write {}:\n   {}", outputfile, e);
            process::exit(1);
        });
        write!(f, "{}", output.join("\n"));
    } else {
        for s in output {
            println!("{}", s);
        }
    }
}
