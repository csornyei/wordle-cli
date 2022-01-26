use rand::{thread_rng, Rng};
use regex::Regex;
use std::fs::File;
use std::io::{BufReader, Read};

pub fn get_progress_row(progress: f32) -> Result<String, String> {
  if progress > 100.0 {
    return Err(String::from("Progress is too high!"));
  }
  let mut progress_blocks: [char; 20] = [' '; 20];
  let full_blocks = (progress / 5.0).floor() as i32;
  for i in 0..full_blocks {
    progress_blocks[i as usize] = char::from_u32(0x2588).unwrap();
  }
  if full_blocks < 20 {
    progress_blocks[full_blocks as usize] = match (progress as i32) % 5 {
      1 => char::from_u32(0x258E).unwrap(),
      2 => char::from_u32(0x258D).unwrap(),
      3 => char::from_u32(0x258B).unwrap(),
      4 => char::from_u32(0x2589).unwrap(),
      _ => ' ',
    };
  }
  let progress_bar: String = progress_blocks.iter().collect();
  Ok(format!("|{}| {:.1}%", progress_bar, progress))
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
