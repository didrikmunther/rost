mod lexer;

fn main() {
    let lexed = lexer::lex("
        let a = \"abc\";
    ");

    println!("\n{:?}", lexed);
}
