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
      if *user_input == *answer {
        for (idx, input_char) in input_chars.iter().enumerate() {
          word_characters[idx] = ColoredChar(input_char.clone(), Color::Green);
        }
        Word {
          character: word_characters,
        }
      } else {
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

#[cfg(test)]
mod test {
  use super::Word;
  use termcolor::Color;

  #[test]
  #[should_panic]
  fn new_invalid_word() {
    let too_long_word = String::from("abcdefg");
    let answer = String::from("edcba");
    Word::new(&too_long_word, &answer);
  }

  #[test]
  fn word_no_char_from_answer() {
    let word_str = String::from("sssss");
    let answer = String::from("aaaaa");

    let word = Word::new(&word_str, &answer);

    let word_chars: Vec<char> = word_str.chars().collect();
    for (idx, col_char) in word.character.iter().enumerate() {
      assert_eq!(col_char.0, word_chars[idx]);
      assert_eq!(col_char.1, Color::Red);
    }

    assert_eq!(word.is_winner(), false);
  }

  #[test]
  fn word_chars_wrong_order() {
    let word_str = String::from("abcde");
    let answer = String::from("edabc");

    let word = Word::new(&word_str, &answer);

    let word_chars: Vec<char> = word_str.chars().collect();
    for (idx, col_char) in word.character.iter().enumerate() {
      assert_eq!(col_char.0, word_chars[idx]);
      assert_eq!(col_char.1, Color::Yellow);
    }

    assert_eq!(word.is_winner(), false);
  }

  #[test]
  fn word_same_as_answer() {
    let word_str = String::from("abcde");
    let answer = String::from("abcde");

    let word = Word::new(&word_str, &answer);

    let word_chars: Vec<char> = word_str.chars().collect();
    for (idx, col_char) in word.character.iter().enumerate() {
      assert_eq!(col_char.0, word_chars[idx]);
      assert_eq!(col_char.1, Color::Green);
    }

    assert_eq!(word.is_winner(), true);
  }

  #[test]
  fn word_has_repeating_characters() {
    let word_str = String::from("knack");
    let answer = String::from("knack");

    let word = Word::new(&word_str, &answer);

    let word_chars: Vec<char> = word_str.chars().collect();
    for (idx, col_char) in word.character.iter().enumerate() {
      assert_eq!(col_char.0, word_chars[idx]);
      assert_eq!(col_char.1, Color::Green);
    }

    assert_eq!(word.is_winner(), true);
  }
}
