#![feature(trace_macros)]
#![feature(proc_macro_hygiene)]

extern crate brainfuck_procmacro;

trace_macros!(true);

use brainfuck_procmacro::brainfuck;

fn main() {
    let s = brainfuck!(+[-[<<[+[--->]-[<<<]]]>>>-]>-.---.>..>.<<<<-.<+.>>>>>.>.<<.<-.);
    println!("{}", s);
}
