use clap::{load_yaml, value_t, App};
use image::{imageops, GenericImageView, ImageError};
use img2ascii::image2ascii;
use std::fs::OpenOptions;
use std::io::{self, prelude::*};

enum Error {
    OutputError(io::Error),
    InputError(ImageError),
}

fn main() {
    match run() {
        Err(Error::OutputError(err)) => eprintln!("output error:\n  {}", err),
        Err(Error::InputError(err)) => eprintln!("input error:\n  {}", err),
        Ok(_) => std::process::exit(0),
    }
    std::process::exit(1);
}

fn run() -> Result<(), Error> {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let input = matches.value_of("filename").unwrap();
    let width = value_t!(matches.value_of("width"), u32).unwrap_or(0);
    let height = value_t!(matches.value_of("height"), u32).unwrap_or(0);

    let image = image::open(input).map_err(Error::InputError)?;

    let (img_w, img_h) = image.dimensions();

    let (width, height) = match (width, height) {
        (0, 0) => (80, 80 * img_h / img_w),
        (0, h) => (h * img_w / img_h, h),
        (w, 0) => (w, w * img_h / img_w),
        (w, h) => (w, h),
    };

    let image = image.resize(width, height, imageops::Nearest).to_luma();

    let mut output = match matches.value_of("output") {
        Some(name) => {
            let file = OpenOptions::new()
                .write(true)
                .create(matches.occurrences_of("force") == 0)
                .open(name)
                .map_err(Error::OutputError)?;
            Ok(Box::new(file) as Box<Write>)
        }
        None => Ok(Box::new(io::stdout()) as Box<Write>),
    }?;

    for s in image2ascii(&image) {
        write!(output, "{}", s).map_err(Error::OutputError)?;
    }
    Ok(())
}
