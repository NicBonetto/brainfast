#[derive(Debug, PartialEq, Copy, Clone)]
enum Token {
    Add,        // +
    Sub,        // -
    Right,      // >
    Left,       // <
    Read,       // ,
    Write,      // .
    BeginLoop,  // [
    EndLoop,    // ]
}
use self::Token::*;

fn main() {
    println!("It's a Brainf*uck to C compiler!");
}
