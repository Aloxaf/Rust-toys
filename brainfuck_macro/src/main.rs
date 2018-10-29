use std::io;
use std::io::prelude::*;

macro_rules! brainfuck {
    (@start $($e:tt)*) => {
        let mut stack = vec![0u8; 256];
        let mut index = 0;
        let mut output = Vec::new();
        brainfuck!(@eval false; (stack, index, output); $($e)*);
    };
    (@eval $lp:expr; ($stack:expr, $index:expr, $output:expr); + $($tail:tt)*) => {
        $stack[$index] = $stack[$index].wrapping_add(1);
        brainfuck!(@eval $lp; ($stack, $index, $output); $($tail)*);
    };
    (@eval $lp:expr; ($stack:expr, $index:expr, $output:expr); - $($tail:tt)*) => {
        $stack[$index] = $stack[$index].wrapping_sub(1);
        brainfuck!(@eval $lp; ($stack, $index, $output); $($tail)*);
    };
    (@eval $lp:expr; ($stack:expr, $index:expr, $output:expr); > $($tail:tt)*) => {
        $index += 1;
        brainfuck!(@eval $lp; ($stack, $index, $output); $($tail)*);
    };
    (@eval $lp:expr; ($stack:expr, $index:expr, $output:expr); < $($tail:tt)*) => {
        $index -= 1;
        brainfuck!(@eval $lp; ($stack, $index, $output); $($tail)*);
    };
    (@eval $lp:expr; ($stack:expr, $index:expr, $output:expr); , $($tail:tt)* ) => {
        io::stdin().read(&mut $stack[$index..$index+1]);
        brainfuck!(@eval $lp; ($stack, $index, $output); $($tail)*);
    };
    (@eval $lp:expr; ($stack:expr, $index:expr, $output:expr); . $($tail:tt)*) => {
        $output.push($stack[$index]);
        brainfuck!(@eval $lp; ($stack, $index, $output); $($tail)*);
    };
    (@eval $lp:expr; ($stack:expr, $index:expr, $output:expr); [ $($body:tt)* ] $($tail:tt)* ) => {
        while $stack[$index] != 0 {
            brainfuck!(@eval true; ($stack, $index, $output); $($body)*);
        }
        brainfuck!(@eval $lp; ($stack, $index, $output); $($tail)*);
    };
    (@eval $lp:expr; ($stack:expr, $index:expr, $output:expr);) => {
        if !$lp {
            println!("{}", String::from_utf8($output.clone()).unwrap());
        }
    };
    ($($e:tt)*) => {
        brainfuck!(@start $($e)*);
    };
}

fn main() {
    brainfuck!(+ [ - [ < < [ + [ - - - > ] - [ < < < ] ] ] > > > - ] >
        - . - - -. > . . > . < < < < - . < + . > > > > > . > . < < . < - .);
}
