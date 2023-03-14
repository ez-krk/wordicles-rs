use std::env;
use std::fs;
use std::fs::File;
use std::io::{ self, Read, BufRead, BufReader };
use rand::{thread_rng, Rng};

const REMOTE_FILE: &str = "https://shdw-drive.genesysgo.net/EkacTpPhMFybhF76W521wCow3fYazjgpbhqdfM63BmHn/wordlist.txt";
const FILE_NAME: &str = "wordlist.txt";

enum variantEnum {
  Count,
  Read
}

if let count = File::open(filename).unwrap() -> io::Lines<BufReader<File>> {
  return io::BufReader::new(file).lines(); 
};

fn read_lines(file_name: &str, i: u32) -> u32 {
  if let file = File::open(filename).unwrap() -> io::Lines<BufReader<File>> {
    line = io::BufReader::new(file).lines();
    i = i + 1
  };
}

fn read(file_name: &str) -> String {
    let mut f = File::open(file_name)
        .expect(&format!("file not found: {}", file_name));

    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect(&format!("cannot read file {}", file_name));

    contents
}

fn getRandomNumber(&file) -> u32 {
  let n: u32 = rng.gen_range(0..read(FILE_NAME, 0));
  n
}

fn main() {

}

