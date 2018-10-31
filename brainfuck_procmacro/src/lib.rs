#![feature(extern_crate_item_prelude)]
#![feature(trace_macros)]
extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::TokenStream;

trace_macros!(true);

#[proc_macro]
pub fn brainfuck(input: TokenStream) -> TokenStream {
    fn calc_offset(code: &[u8], rev: bool) -> usize {
        let mut code = code.to_owned();
        if rev {
            code.reverse();
        }
        code.iter().map(|&c| match c {
            b'[' => 1,
            b']' => -1,
            _ => 0,
        }).scan(0, |s, c| {
            *s += c;
            Some(*s)
        }).enumerate().skip_while(|&(_i, c)| {
            c != 0
        }).next().unwrap().0
    }

    fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
        let mut input = input.iter();
        let mut stack: Vec<u8> = Vec::new();
        let mut output: Vec<u8> = Vec::new();
        let (mut index, mut code_index): (usize, usize) = (0, 0);
        let len = code.len();
        let code: Vec<u8> = code.chars().map(|x| x as u8).collect();

        stack.resize(15, 0);
        while code_index < len {
            let c = code[code_index];

            if stack.len() <= index {
                stack.resize(index + 1, 0);
            }
            match c {
                b'>' => index += 1,
                b'<' => index -= 1,
                b'+' => stack[index] = stack[index].wrapping_add(1),
                b'-' => stack[index] = stack[index].wrapping_sub(1),
                b',' => stack[index] = *input.next().unwrap(),
                b'.' => output.push(stack[index]),
                b'[' => {
                    if stack[index] == 0 {
                        code_index += calc_offset(&code[code_index..], false);
                    }
                }
                b']' => {
                    if stack[index] != 0 {
                        code_index -= calc_offset(&code[..=code_index], true);
                    }
                }
                _ => (),
            }
            code_index += 1;
        }
        output
    }

    let n = input.to_string();
    let b = brain_luck(&n, vec![]);
    format!(r#""{}""#, String::from_utf8(b).unwrap()).parse().unwrap()
}
