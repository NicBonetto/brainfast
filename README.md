# brainfast
A simple Brainf*ck ⇸ C CLI compiler.

## Download
Clone the repository into any directory using the following command:
```
$ git clone https://github.com/NicBonetto/brainfast.git
$ cd brainfast/
$ cargo build --release
```

##### TODO: Set PATH 
For now, you have to manually set the PATH variable so `brainfast` can be used outside of the 
project directory. If you are working on a Unix-like OS, follow these commands once you are in the root 
directory for the repository:
```
// List PATH values
$ echo $PATH | tr \: \\n
// Copy binary to a PATH value
$ cp ./target/release/brainfast <PATH>
```
Or, if you like, you can set your own PATH and create new folders. 

## Usage
```
$ brainfast -h
It's a Brainf*ck to C compiler!!!


brainfast 1.0.0
Nic Bonetto <nick.bonetto@gmail.com>
A Brainfuck to C compiler written in Rust

USAGE:
    brainfast <PATH> <TARGET>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <PATH>      target brainfuck file
    <TARGET>    target C file
```
