use self::Instr::*;
use lavan::prelude::*;
use std::io::Read;

#[derive(Debug, Clone)]
enum Instr {
    Left,
    Right,
    Incr,
    Decr,
    Read,
    Write,
    Loop(Vec<Self>),
}

fn parse(input: &mut std::str::Chars) -> Sure<Vec<Instr>> // Sure<T> <=> Result<T, Infallible>
{
    // due to Instr::Loop
    recursive(|parser| {
        // match into the next character
        any_then(|c| {
            Some(match c {
                // match into it
                '<' => Left,
                '>' => Right,
                '+' => Incr,
                '-' => Decr,
                ',' => Read,
                '.' => Write,
                _ => return None, // try the next approach
            })
        })
        // or recursively parse the loop
        .or(parser.delimited('[', ']').map(Loop))
        .catch(
            // since this is parser is recursive, we need
            // an exit condition, it being ']'
            any_ne(']').del(), // up to this point, ignore all characters except for ']'
        )
        .repeat() // repeat this process again and again
        .collect() // collect all instructions into the Vec
    })
    .parse_once(input) // apply this parser to the input
}

const TAPE_LEN: usize = 10_000;

fn execute(instrs: &[Instr], cursor: &mut usize, tape: &mut [u8; TAPE_LEN]) {
    for instr in instrs {
        match instr {
            Right => *cursor += 1,
            Left => *cursor -= 1,
            Incr => tape[*cursor] += 1,
            Decr => tape[*cursor] -= 1,
            Write => print!("{}", tape[*cursor] as char),
            Read => {
                let mut input: [u8; 1] = [0; 1];
                std::io::stdin()
                    .read_exact(&mut input)
                    .expect("failed to read stdin");
                tape[*cursor] = input[0];
            }
            Loop(instrs) => {
                while tape[*cursor] != 0 {
                    execute(&instrs, cursor, tape)
                }
            }
        }
    }
}

fn main() {
    let input = "This should print 'Hello World' ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

    let instrs = parse(&mut input.chars()).value();

    execute(&instrs, &mut 0, &mut [0; TAPE_LEN]);
}
