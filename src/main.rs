pub mod lib;
use text_io::read;
use termcolor::{Color};
use lib::ColoredChar; 

fn main() {
    let guesses = match lib::get_guesses() {
        Err(_) => {
            println!("Error! Can't find allowed guesses!");
            std::process::exit(1);
        },
        Ok(guesses) => guesses
    };

    let answer = lib::choose_answer();

    print!("Choosen word ");
    lib::color_write(&answer, Color::Green);
    print!("\n");

    let mut entered_words = 0;

    let mut guessed_words: Vec<lib::Word> = vec!();

    let mut remaining_letters: Vec<ColoredChar> = ColoredChar::get_default_keys();
    let mut wrong_letters: Vec<ColoredChar> = vec![];
    let mut guessed_letters: Vec<ColoredChar> = vec![];

    while entered_words < 5 {
        lib::print_screen(&guessed_words, &remaining_letters, &guessed_letters, &wrong_letters);
        // Getting user input:
        let mut user_input: String;
        loop {
            user_input = read!("{}\n");
            if lib::validate_user_input(&user_input, &guesses) {
                break;
            } else {
                println!("Not valid word!");
            }
        }

        let user_word = lib::Word::new(&user_input, &answer);

        if user_word.is_winner() {
            println!("You won! You needed {} guesses!", guessed_words.len() + 1);
            break;
        }
        user_word.update_letters(&mut remaining_letters, &mut guessed_letters, &mut wrong_letters);
        guessed_words.push(user_word);
        entered_words += 1;
    }
}
