extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/X11logger/X11logger.cpp")
        .cpp(true)
        .opt_level(2)
        .compile("X11logger");
}