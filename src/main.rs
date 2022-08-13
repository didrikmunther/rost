mod lexer;

fn main() {
    let lexed = lexer::lex("
        5 + 5 - 2
    ");

    println!("\n{:?}", lexed);
}
