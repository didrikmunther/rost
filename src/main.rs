use std::io;

mod lexer;

fn main() {
    let lexed = lexer::lex("
        let ab = 5 ; let k = true;
    ");

    println!("\n{:?}", lexed);
}
