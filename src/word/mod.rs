use crate::colored_char::ColoredChar;
use termcolor::Color;

#[derive(Copy, Clone)]
pub struct Word {
  pub character: [ColoredChar; 5],
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
          }
          None => {
            word_characters[idx] = ColoredChar(input_char.clone(), Color::Red);
          }
        }
      }
      Word {
        character: word_characters,
      }
    }
  }

  pub fn print_word(&self) {
    for col_char in self.character {
      col_char.print();
    }
    println!("");
  }

  pub fn is_winner(&self) -> bool {
    for col_char in self.character.iter() {
      if col_char.1 != Color::Green {
        return false;
      }
    }
    true
  }
}
