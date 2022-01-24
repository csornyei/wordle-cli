use std::fs::File;
use rand::{Rng, thread_rng};
use std::io::{Write, BufReader, Read};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Copy, Clone)]
pub struct Word {
  character: [(char, Color); 5]
}

impl Word {
  pub fn new(user_input: &String, answer: &String) -> Word {
    let input_chars: Vec<char> = user_input.chars().collect();
    let answer_chars: Vec<char> = answer.chars().collect();
    if input_chars.len() != 5 {
      panic!("Not valid word!");
    } else {
      let mut word_characters: [(char, Color); 5] = [('a', Color::Red); 5];
      for (idx, input_char) in input_chars.iter().enumerate() {
        match answer_chars.iter().position(|&c| c == *input_char) {
          Some(ans_idx) => {
            if idx == ans_idx {
              word_characters[idx] = (input_char.clone(), Color::Green);
            } else {
              word_characters[idx] = (input_char.clone(), Color::Yellow);
            }
          },
          None => {
            word_characters[idx] = (input_char.clone(), Color::Red);
          }
        }
      };
      Word { character: word_characters}
    }
  }

  pub fn print_word(&self) {
    for (c, col) in self.character {
      let mut b = [0; 4];
      color_write(c.encode_utf8(&mut b), col);
    }
    println!("");
  }

  pub fn is_winner(&self) -> bool {
    for (_, color) in self.character.iter() {
      if *color != Color::Green {
        return false
      }
    };
    true
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
  let guesses: Vec<String> = content.split("\n").map(|x| String::from(x)).collect();
  Ok(guesses)
}

pub fn print_screen(guessed_words: &Vec<Word>) {
  print!("\x1B[2J\x1b[1;1H");
  println!("Words so far:");
  for word in guessed_words.iter() {
    word.print_word();
  }

  println!("Remaining letters:");

  println!("Wrong letters:");

  println!("Guessed letters");

  println!("Please provide a new word!");
}

pub fn validate_user_input(input: &String, allowed_guesses: &Vec<String>) -> bool {
  if input.len() != 5 {
    false
  } else {
    allowed_guesses.contains(&input.clone().to_lowercase())
  }
}