mod lexer;

fn main() {
    let lexed = lexer::lex("
        let a = 5;
        let b = a + 1 - 2;
    ");

    println!("\n{:?}", lexed);
}
