mod lexer;

fn main() {
    let lexed = lexer::lex("
        let a = 5;
        let b = 1;
    ");

    println!("\n{:?}", lexed);
}
