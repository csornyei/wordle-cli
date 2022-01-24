use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn color_write(text: &str, color: Color) {
  let mut stdout = StandardStream::stdout(ColorChoice::Always);
  stdout
    .set_color(ColorSpec::new().set_fg(Some(color)))
    .expect("Error with setting color!");
  match write!(&mut stdout, "{}", text) {
    Err(_) => std::process::exit(1),
    Ok(_) => {}
  };
  stdout
    .set_color(ColorSpec::new().set_fg(Some(Color::White)))
    .expect("Error resetting terminal color!");
}

#[derive(Copy, Clone)]
pub struct ColoredChar(pub char, pub Color);

impl ColoredChar {
  pub fn new_white(c: char) -> ColoredChar {
    ColoredChar(c, Color::White)
  }

  pub fn get_default_keys() -> Vec<ColoredChar> {
    let keys = vec![
      'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k',
      'l', 'z', 'x', 'c', 'v', 'b', 'n', 'm',
    ];
    let mut colored_chars: Vec<ColoredChar> = vec![];
    for c in keys.iter() {
      colored_chars.push(ColoredChar::new_white(*c));
    }
    colored_chars
  }

  pub fn print(&self) {
    let mut b = [0; 4];
    color_write(self.0.encode_utf8(&mut b), self.1)
  }
}
