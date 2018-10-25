extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/X11logger/X11logger.c")
        .flag("-lX11")
        .compile("X11logger");
}