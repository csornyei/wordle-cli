pub mod colored_char;
pub mod lib;
pub mod word;
use colored_char::ColoredChar;
use termcolor::Color;
use text_io::read;
use word::Word;

struct GameState {
    win: bool,
    guessed_words: Vec<Word>,
    remaining_letters: Vec<ColoredChar>,
    wrong_letters: Vec<ColoredChar>,
    guessed_letters: Vec<ColoredChar>,
    answer: String,
    guesses: Vec<String>,
}

impl GameState {
    pub fn init_game() -> GameState {
        GameState {
            win: false,
            guessed_words: vec![],
            remaining_letters: ColoredChar::get_default_keys(),
            wrong_letters: vec![],
            guessed_letters: vec![],
            answer: String::new(),
            guesses: vec![],
        }
    }

    pub fn get_entered_words(&self) -> i8 {
        self.guessed_words.len() as i8
    }

    pub fn new_game(&mut self) {
        self.win = false;
        self.guessed_words = vec![];
        self.remaining_letters = ColoredChar::get_default_keys();
        self.wrong_letters = vec![];
        self.guessed_letters = vec![];
        self.choose_answer();
    }

    fn choose_answer(&mut self) {
        self.answer = lib::choose_answer();
    }

    pub fn load_guesses(&mut self) {
        self.guesses = match lib::get_guesses() {
            Err(_) => {
                println!("Error! Can't find allowed guesses!");
                std::process::exit(1);
            }
            Ok(guesses) => guesses,
        }
    }

    pub fn print_screen(&self) {
        print!("\x1B[2J\x1b[1;1H");
        println!("Words so far:");
        for word in self.guessed_words.iter() {
            word.print_word();
        }

        print!("Remaining letters:");
        for c in self.remaining_letters.clone() {
            c.print();
            print!(", ");
        }

        println!("\nWrong letters:");
        for c in self.wrong_letters.clone() {
            c.print();
            print!(", ");
        }

        println!("\nGuessed letters");
        for c in self.guessed_letters.clone() {
            c.print();
            print!(", ");
        }

        println!("\nPlease provide a new word!");
    }

    pub fn request_word(&mut self) {
        let mut user_input: String;
        loop {
            user_input = read!("{}\n");
            if lib::validate_user_input(&user_input, &self.guesses) {
                break;
            } else {
                println!("Not valid word!");
            }
        }
        let user_word = Word::new(&user_input, &self.answer);
        self.enter_word(user_word);
    }

    pub fn prompt_exit(&self) -> bool {
        println!("Do you want to play again? [Y/n]");
        let user_input: String;
        user_input = read!("{}\n");
        if user_input.len() == 0 {
            return true;
        }
        let input_char = user_input.to_lowercase().chars().nth(0).unwrap();
        input_char == 'n'
    }

    fn update_letters(&mut self, word: &Word) {
        for colored_char in word.character {
            match self
                .remaining_letters
                .iter()
                .position(|x| x.0 == colored_char.0)
            {
                None => {}
                Some(p) => {
                    self.remaining_letters.remove(p);
                    if colored_char.1 == Color::Red {
                        self.wrong_letters.push(colored_char.clone());
                    } else {
                        self.guessed_letters.push(colored_char.clone());
                    }
                }
            };
            match self
                .guessed_letters
                .iter()
                .position(|x| x.0 == colored_char.0)
            {
                None => {}
                Some(p) => {
                    if colored_char.1 == Color::Green && self.guessed_letters[p].1 != Color::Green {
                        self.guessed_letters.remove(p);
                        self.guessed_letters.push(colored_char.clone());
                    }
                }
            };
        }
    }

    fn enter_word(&mut self, word: Word) {
        self.guessed_words.push(word);
        self.win = word.is_winner();
        if !self.win {
            self.update_letters(&word);
        }
    }
}

fn main() {
    let mut state = GameState::init_game();
    state.load_guesses();
    loop {
        state.new_game();
        while state.get_entered_words() < 5 {
            state.print_screen();
            state.request_word();
            if state.win {
                println!("You won after {} guess!", state.get_entered_words());
                break;
            }
        }
        if !state.win {
            println!("You lose! The secret word was {}", state.answer);
        }
        if state.prompt_exit() {
            println!("Goodbye!");
            break;
        }
    }
}
