mod lexer;

fn main() {
    let lexed = lexer::lex("
        let a = 5;
    ");

    println!("\n{:?}", lexed);
}
