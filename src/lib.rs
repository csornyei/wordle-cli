use std::fs::File;
use rand::{Rng, thread_rng};
use std::io::{Write, BufReader, Read};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Copy, Clone)]
pub struct ColoredChar (char, Color);

impl ColoredChar {
  pub fn new_white(c: char) -> ColoredChar {
    ColoredChar(c, Color::White)
  }

  pub fn get_default_keys() -> Vec<ColoredChar> {
    let keys = vec!['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'z', 'x', 'c', 'v', 'b', 'n', 'm'];
    let mut colored_chars: Vec<ColoredChar> = vec![]; 
    for c in keys.iter() {
      colored_chars.push(ColoredChar::new_white(*c));
    };
    colored_chars
  }

  pub fn print(&self) {
    let mut b = [0; 4];
    color_write(self.0.encode_utf8(&mut b), self.1)
  }
}

#[derive(Copy, Clone)]
pub struct Word {
  character: [ColoredChar; 5]
}

impl Word {
  pub fn new(user_input: &String, answer: &String) -> Word {
    let input_chars: Vec<char> = user_input.chars().collect();
    let answer_chars: Vec<char> = answer.chars().collect();
    if input_chars.len() != 5 {
      panic!("Not valid word!");
    } else {
      let mut word_characters: [ColoredChar; 5] = [ColoredChar('a', Color::Red); 5];
      for (idx, input_char) in input_chars.iter().enumerate() {
        match answer_chars.iter().position(|&c| c == *input_char) {
          Some(ans_idx) => {
            if idx == ans_idx {
              word_characters[idx] = ColoredChar(input_char.clone(), Color::Green);
            } else {
              word_characters[idx] = ColoredChar(input_char.clone(), Color::Yellow);
            }
          },
          None => {
            word_characters[idx] = ColoredChar(input_char.clone(), Color::Red);
          }
        }
      };
      Word { character: word_characters}
    }
  }

  pub fn print_word(&self) {
    for col_char in self.character {
      col_char.print();
    }
    println!("");
  }

  pub fn is_winner(&self) -> bool {
    for colored_char in self.character.iter() {
      if colored_char.1 != Color::Green {
        return false
      }
    };
    true
  }

  pub fn update_letters(&self, remaining_letters: &mut Vec<ColoredChar>, guessed_letters: &mut Vec<ColoredChar>, wrong_letters: &mut Vec<ColoredChar>) {
    for colored_char in self.character {
      match remaining_letters.iter().position(|x| x.0 == colored_char.0) {
        None => {},
        Some(p) => {
          remaining_letters.remove(p);
          if colored_char.1 == Color::Red {
            wrong_letters.push(colored_char.clone());
          } else {
            guessed_letters.push(colored_char.clone());
          }
        }
      }
    }
  }
}

pub fn color_write(text: &str, color: Color) {
  let mut stdout = StandardStream::stdout(ColorChoice::Always);
  stdout.set_color(ColorSpec::new().set_fg(Some(color))).expect("Error with setting color!");
  match write!(&mut stdout, "{}", text) {
    Err(_) => std::process::exit(1),
    Ok(_) => {}
  };
  stdout.set_color(ColorSpec::new().set_fg(Some(Color::White))).expect("Error resetting terminal color!");
}

fn get_answers() -> std::io::Result<Vec<String>> {
  let file = File::open("answers.txt")?;
  let mut reader = BufReader::new(file);
  let mut content = String::new();
  reader.read_to_string(&mut content)?;
  let guesses: Vec<String> = content.split("\n").map(|x| String::from(x)).collect();
  Ok(guesses)
}

pub fn choose_answer() -> String {
  let answers = match get_answers() {
    Err(_) => {
            println!("Error! Can't find answers!");
            std::process::exit(1);
    },
    Ok(answers) => answers
  };
  let mut rng = thread_rng();
  let answer_idx = rng.gen_range(0..=answers.len());
  answers[answer_idx].clone()
}

pub fn get_guesses() -> std::io::Result<Vec<String>> {
  let file = File::open("allowed_guesses.txt")?;
  let mut reader = BufReader::new(file);
  let mut content = String::new();
  reader.read_to_string(&mut content)?;
  let mut guesses: Vec<String> = content.split("\n").map(|x| String::from(x)).collect();
  let mut answers = get_answers()?;
  guesses.append(&mut answers);
  Ok(guesses)
}

pub fn print_screen(guessed_words: &Vec<Word>, remaining_letters: &Vec<ColoredChar>, guessed_letters: &Vec<ColoredChar>, wrong_letters: &Vec<ColoredChar>) {
  print!("\x1B[2J\x1b[1;1H");
  println!("Words so far:");
  for word in guessed_words.iter() {
    word.print_word();
  }

  print!("Remaining letters:");
  for c in remaining_letters {
    c.print();
    print!(", ");
  }

  println!("\nWrong letters:");
    for c in wrong_letters {
    c.print();
    print!(", ");
  }

  println!("\nGuessed letters");
  for c in guessed_letters {
    c.print();
    print!(", ");
  }

  println!("\nPlease provide a new word!");
}

pub fn validate_user_input(input: &String, allowed_guesses: &Vec<String>) -> bool {
  if input.len() != 5 {
    false
  } else {
    allowed_guesses.contains(&input.clone().to_lowercase())
  }
}