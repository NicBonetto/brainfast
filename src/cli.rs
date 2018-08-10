extern crate clap;

use self::clap::{App, Arg};

pub fn app() -> Vec<String> {
  let matches = App::new("brainfast")
    .version("1.0.0")
    .author("Nic Bonetto <nick.bonetto@gmail.com>")
    .about("A Brainfuck to C compiler written in Rust")
    .arg(Arg::with_name("PATH")
      .required(true)
      .takes_value(true)
      .index(1)
      .help("target brainfuck file"))
    .arg(Arg::with_name("TARGET")
      .required(true)
      .takes_value(true)
      .index(2)
      .help("target C file"))
    .get_matches();

  vec![
    matches.value_of("PATH").unwrap().to_string(),
    matches.value_of("TARGET").unwrap().to_string()
  ]
}
