extern crate clap;

use self::clap::{App, Arg};
use std::path::Path;
use std::ffi::OsStr;

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

    let path = matches.value_of("PATH").unwrap().to_string();
    let target = matches.value_of("TARGET").unwrap().to_string();

    // Check file extensions
    if Path::new(&path).extension().and_then(OsStr::to_str) != Some("bf") {
      panic!("PATH must have .bf extension.");
    }
    if Path::new(&target).extension().and_then(OsStr::to_str) != Some("c") {
      panic!("TARGET must have .c extension.");
    }

  vec![
    path,
    target
  ]
}
