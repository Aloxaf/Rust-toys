#![feature(proc_macro_hygiene, plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate brainfuck;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use self::brainfuck::Interpreter;
use rocket_contrib::Json;
use std::io::prelude::*;
use std::io::Cursor;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Serialize, Deserialize)]
struct BrainFuck {
    stack_size: u16,
    input: String,
    code: String,
}

#[derive(Serialize, Deserialize)]
struct BrainFuckResult {
    success: bool,
    output: String,
}

#[post("/brainfuck", format = "application/json", data = "<bf>")]
fn eval_bf(bf: Json<BrainFuck>) -> Json<BrainFuckResult> {
    let bf: BrainFuck = bf.into_inner();
    let input: Cursor<String> = Cursor::new(bf.input);
    let mut output: Vec<u8> = Vec::new();
    let mut interpreter: Interpreter<Cursor<String>, &mut Vec<u8>> =
        Interpreter::new(bf.stack_size as usize, input, output.as_mut());

    let ret = match interpreter.eval(&bf.code) {
        Ok(()) => BrainFuckResult {
            success: true,
            output: String::from_utf8(output).unwrap(),
        },
        Err(err) => BrainFuckResult {
            success: false,
            output: err.to_string(),
        },
    };
    Json(ret)
}

// Oh no!
// https://stackoverflow.com/questions/43424982/how-to-parse-multipart-forms-using-abonander-multipart-with-rocket/43427509#43427509
//#[post("/miragetank", format = "application/json", data = "<images>")]
//fn build_car() {
//
//}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, eval_bf])
        .launch();
}
