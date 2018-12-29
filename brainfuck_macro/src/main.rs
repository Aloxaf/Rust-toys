macro_rules! brainfuck {
    (@start $($e:tt)*) => {
    {
        let mut stack = vec![0u8; 256];
        let mut index = 0;
        let mut output = Vec::new();
        brainfuck!(@eval (stack, index, output); $($e)*);
        String::from_utf8(output.clone()).unwrap()
    }
    };
    (@eval ($stack:expr, $index:expr, $output:expr); + $($tail:tt)*) => {
        $stack[$index] = $stack[$index].wrapping_add(1);
        brainfuck!(@eval ($stack, $index, $output); $($tail)*);
    };
    (@eval ($stack:expr, $index:expr, $output:expr); - $($tail:tt)*) => {
        $stack[$index] = $stack[$index].wrapping_sub(1);
        brainfuck!(@eval ($stack, $index, $output); $($tail)*);
    };
    (@eval ($stack:expr, $index:expr, $output:expr); > $($tail:tt)*) => {
        $index += 1;
        brainfuck!(@eval ($stack, $index, $output); $($tail)*);
    };
    (@eval ($stack:expr, $index:expr, $output:expr); < $($tail:tt)*) => {
        $index -= 1;
        brainfuck!(@eval ($stack, $index, $output); $($tail)*);
    };
    (@eval ($stack:expr, $index:expr, $output:expr); , $($tail:tt)* ) => {
        io::stdin().read(&mut $stack[$index..$index+1]);
        brainfuck!(@eval ($stack, $index, $output); $($tail)*);
    };
    (@eval ($stack:expr, $index:expr, $output:expr); . $($tail:tt)*) => {
        $output.push($stack[$index]);
        brainfuck!(@eval ($stack, $index, $output); $($tail)*);
    };
    (@eval ($stack:expr, $index:expr, $output:expr); [ $($body:tt)* ] $($tail:tt)* ) => {
        while $stack[$index] != 0 {
            brainfuck!(@eval ($stack, $index, $output); $($body)*);
        }
        brainfuck!(@eval ($stack, $index, $output); $($tail)*);
    };
    (@eval ($stack:expr, $index:expr, $output:expr);) => { };
    ($($e:tt)*) => {
        brainfuck!(@start $($e)*);
    };
}

fn main() {
    let s = brainfuck!(+ [ - [ < < [ + [ - - - > ] - [ < < < ] ] ] > > > - ] >
        - . - - -. > . . > . < < < < - . < + . > > > > > . > . < < . < - .);
    println!("{}", s);
}
