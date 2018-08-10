use std::fs::{File, OpenOptions};
use std::io::*;

mod cli;
mod compile;

use cli::app;
use compile::{tokenize, generate};

fn main() {
  println!("It's a Brainf*ck to C compiler!!!\n\n");

  let matches = app();
  let mut read_buffer = File::open(&matches[0])
    .expect("Could not read Brainfuck file.");

  let mut contents = String::new();
  read_buffer.read_to_string(&mut contents)
    .expect("Something went wrong reading the Brainfuck file.");

  let tokens = tokenize(&contents);
  let code = generate(&tokens);

  let mut options = OpenOptions::new();
  options.write(true);

  let write_file = match options.open(&matches[1]) {
    Ok(file) => file,
    Err(_) => panic!("No target C file found!"),
  };
  let mut writer = BufWriter::new(&write_file);
  writer.write_all(code.as_bytes()).unwrap();

  println!("Finished compiling!");
}
