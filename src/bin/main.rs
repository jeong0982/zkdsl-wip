extern crate rustyline;
extern crate zkdsl;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use zkdsl::lexer::lexer::Lexer;
use zkdsl::token::token::Token;

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
