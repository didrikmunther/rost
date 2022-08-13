mod lexer;

fn main() {
    let lexed = lexer::lex("
        // here
        let a = 5;
    ");

    println!("\n{:?}", lexed);
}
