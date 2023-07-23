extern crate zkdsl;
extern crate rustyline;

use zkdsl::lexer::lexer::Lexer;
use zkdsl::token::token::Token;
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let mut rl = Editor::<()>::new();
    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                rl.add_history_entry(&line);

                let mut lexer = Lexer::new(&line);

                loop {
                    let tok = lexer.next_token();
                    if tok == Token::Eof {
                        break;
                    }

                    println!("{:?}", tok);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("\nInterrupted");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!();
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }

}
