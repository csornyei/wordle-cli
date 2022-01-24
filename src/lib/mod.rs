use rand::{thread_rng, Rng};
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
  if input.len() != 5 {
    false
  } else {
    allowed_guesses.contains(&input.clone().to_lowercase())
  }
}
