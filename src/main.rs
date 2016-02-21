extern crate compilers;
extern crate argparse;
use argparse::{ArgumentParser, Store, StoreTrue, StoreFalse};
use compilers::lexer::{Lexer, Token};

fn main() {
    let mut input = String::new();
    let mut file = true;
    // let mut parse = true;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Compilers Lab");

        ap.refer(&mut file)
          .add_option(&["-f", "--file"], StoreTrue, "If ")
          .add_option(&["-t", "--text"], StoreFalse, "Lex input");
        // ap.refer(&mut parse)
        //         .add_option(&["parse"], StoreTrue, "Parse and lex input")
        //         .add_option(&["lex"], StoreFalse, "Lex input");
        ap.refer(&mut input)
          .add_argument("input", Store, "Input")
          .required();
        ap.parse_args_or_exit();

    }
    if file {
        println!("file {}", input);
    } else {
        let lexer = Lexer::new(input.chars());
        for token in lexer {
            match token {
                Token::Error{pos, message} => {
                    println!("Error {} at position {}", message, pos);
                    break;
                }
                _ => {
                    println!("{:?}", token);
                }
            }

        }
    }
}
