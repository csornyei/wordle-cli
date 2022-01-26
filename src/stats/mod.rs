use crate::lib::get_progress_row;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};

pub struct Stats {
  one: i32,
  two: i32,
  three: i32,
  four: i32,
  five: i32,
  lost: i32,
}

impl Stats {
  pub fn new() -> Stats {
    let stats_vec = Stats::load_stats().unwrap();
    Stats {
      one: stats_vec[0].parse::<i32>().unwrap(),
      two: stats_vec[1].parse::<i32>().unwrap(),
      three: stats_vec[2].parse::<i32>().unwrap(),
      four: stats_vec[3].parse::<i32>().unwrap(),
      five: stats_vec[4].parse::<i32>().unwrap(),
      lost: stats_vec[5].parse::<i32>().unwrap(),
    }
  }

  pub fn save(&self) {
    let stats_vec: Vec<String> = vec![
      self.one, self.two, self.three, self.four, self.five, self.lost,
    ]
    .iter()
    .map(|x| x.to_string())
    .collect();
    match Stats::save_stats(&stats_vec) {
      Err(e) => {
        println!("{}", e);
        panic!("Can't save stats")
      }
      Ok(_) => (),
    };
  }

  pub fn print(&self) {
    let sum = (self.one + self.two + self.three + self.four + self.five + self.lost) as f32;
    println!(
      "1\t: {}",
      get_progress_row((self.one as f32) / sum * 100.0).unwrap()
    );
    println!(
      "2\t: {}",
      get_progress_row((self.two as f32) / sum * 100.0).unwrap()
    );
    println!(
      "3\t: {}",
      get_progress_row((self.three as f32) / sum * 100.0).unwrap()
    );
    println!(
      "4\t: {}",
      get_progress_row((self.four as f32) / sum * 100.0).unwrap()
    );
    println!(
      "5\t: {}",
      get_progress_row((self.five as f32) / sum * 100.0).unwrap()
    );
    println!(
      "Lost\t: {}",
      get_progress_row((self.lost as f32) / sum * 100.0).unwrap()
    );
  }

  pub fn increment(&mut self, key: i8) {
    match key {
      1 => self.one += 1,
      2 => self.two += 1,
      3 => self.three += 1,
      4 => self.four += 1,
      5 => self.five += 1,
      -1 => self.lost += 1,
      _ => panic!("Invalid key!"),
    }
  }

  fn get_stats_file_path() -> PathBuf {
    dirs::home_dir().unwrap().join(Path::new(".wordle-cli"))
  }

  fn load_stats() -> std::io::Result<Vec<String>> {
    let stats_file_path = Stats::get_stats_file_path();
    if stats_file_path.exists() {
      let file = File::open(stats_file_path)?;
      let mut reader = BufReader::new(file);
      let mut content = String::new();
      reader.read_to_string(&mut content)?;
      if content.ends_with("\n") {
        content.pop();
        if content.ends_with("\r") {
          content.pop();
        }
      }
      let stats: Vec<String> = content.split(",").map(|x| String::from(x)).collect();
      Ok(stats)
    } else {
      let stats: Vec<String> = vec!["0", "0", "0", "0", "0", "0"]
        .iter()
        .map(|x| String::from(*x))
        .collect();
      Stats::save_stats(&stats)?;
      Ok(stats)
    }
  }

  fn save_stats(content: &Vec<String>) -> std::io::Result<()> {
    let stats_file_path = Stats::get_stats_file_path();
    let mut file = File::create(stats_file_path)?;
    let file_string = content.join(",");
    let file_content = file_string.as_bytes();
    file.write(file_content)?;
    Ok(())
  }
}
