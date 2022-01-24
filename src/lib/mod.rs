use rand::{thread_rng, Rng};
use regex::Regex;
use std::fs::File;
use std::io::{BufReader, Read};

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
    }
    Ok(answers) => answers,
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

pub fn validate_user_input(input: &String, allowed_guesses: &Vec<String>) -> bool {
  let re = Regex::new(r"^[a-zA-Z]{5}$").unwrap();
  if !re.is_match(input) {
    false
  } else {
    allowed_guesses.contains(&input.clone().to_lowercase())
  }
}

#[cfg(test)]
mod test {
  use super::validate_user_input;

  #[test]
  fn validate_user_input_test() {
    let allowed_guesses = vec![
      String::from("aaaaa"),
      String::from("abbbb"),
      String::from("abccc"),
      String::from("abcdd"),
      String::from("abcde"),
      String::from("*-/.,"),
    ];

    let too_long_input = String::from("abcdef");
    let not_valid_char = String::from("*-/.,");
    let not_in_allowed_guesses = String::from("zxcvb");
    let valid_input = String::from("abcde");

    assert_eq!(
      validate_user_input(&too_long_input, &allowed_guesses),
      false
    );
    assert_eq!(
      validate_user_input(&not_valid_char, &allowed_guesses),
      false
    );
    assert_eq!(
      validate_user_input(&not_in_allowed_guesses, &allowed_guesses),
      false
    );
    assert_eq!(validate_user_input(&valid_input, &allowed_guesses), true);
  }
}
