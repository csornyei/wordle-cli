use std::fs::File;
use rand::{Rng, thread_rng};
use std::io::{Write, BufReader, Read};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

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

pub fn print_screen() {
  println!("Words so far:");
  color_write("WEARY", Color::Red);
  println!("");
  color_write("BU", Color::Red);
  color_write("L", Color::Yellow);
  color_write("GE", Color::Red);
  println!("");
  color_write("P", Color::Red);
  color_write("O", Color::Yellow);
  color_write("I", Color::Red);
  color_write("N", Color::Yellow);
  color_write("T", Color::Red);
  println!("");
  color_write("NO", Color::Yellow);
  color_write("MAD", Color::Red);
  println!("");
  color_write("C", Color::Red);
  color_write("L", Color::Yellow);
  color_write("O", Color::Green);
  color_write("C", Color::Red);
  color_write("K", Color::Yellow);
  println!("");
  color_write("KNOLL", Color::Green);
  println!("");

  println!("Remaining letters:");

  println!("Wrong letters:");

  println!("Guessed letters");

  println!("Please provide a new word!");
}