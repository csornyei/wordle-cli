pub mod lib;
use text_io::read;
use termcolor::{Color};
use crate::lib::{color_write, get_guesses, choose_answer, print_screen};

fn main() {
    let guesses = match get_guesses() {
        Err(_) => {
            println!("Error! Can't find allowed guesses!");
            std::process::exit(1);
        },
        Ok(guesses) => guesses
    };

    let answer = choose_answer();

    print!("Choosen word ");
    color_write(&answer, Color::Green);
    print!("\n");

    print_screen();
    let user_input: String = read!("{}\n");
    println!("Your input is: {}", user_input);
}


/* 
    Program flow:
        - read files
        - choose word
        - get user input                                           <-
            -> validate input                                       |
                -> 5 letters long, only small english letters       |
                -> against available words                          |
            -> is input word the valid word?                        |
            -> check letters in word                                |
                -> is char in word?                                 |
                    -> in right position                            |
        - get user input if not expected word                       |

*/
