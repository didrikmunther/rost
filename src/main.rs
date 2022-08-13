mod lexer;

fn main() {
    let lexed = lexer::lex("let ¢ = 5;");

    println!("\n{:?}", lexed);
}
