use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::SystemTime;
use std::collections::HashSet;
use rand::Rng;
// use uuid::Uuid;

#[derive(Debug)]
struct Wordlist {
    words: HashSet<String>,
    count: u64
}

fn create_wordlist() -> Wordlist {
    let mut i: u64 = 0;
    let mut words = HashSet::<String>::new();
    if let Ok(lines) = read_lines("./wordlist.txt") {
        for line in lines {
            if let Ok(word) = line {
                i = i +1;
                words.insert(word.to_string());           
            }
        }
        println!("wordlist has {i} words.");
    }
    let wordlist = Wordlist {
        words: words,
        count: i
    };
    wordlist
}

fn count_wordlist(wordlist: &Wordlist) -> u64 {
    let mut i = 0;
    for word in &wordlist.words {
        println!("{word}");
        i = i +1
    };
    i
}

#[derive(Debug)]
struct Game<'a> { wordlist: &'a Wordlist, words: u64, active: bool, created_at: SystemTime }

fn create_game(wordlist: &Wordlist) -> Game {
    let game = Game {
        wordlist: wordlist,
        words: count_wordlist(&wordlist),
        active: true,
        created_at: SystemTime::now(),
    };
    game
}

#[derive(Debug)]
struct Word<'a> {
    wordlist: &'a Wordlist,
    word: String,
    length: u64,
    rng: u64,
    active: bool,
    created_at: SystemTime,
}

fn word_length(word : String) -> u64 {
    let num: u64 = word.len().try_into().unwrap();
    num
}


fn new_word<'a>(secret_text: &'a String, game: &'a Game) -> Word<'a> {
    let word = Word {
        wordlist: &game.wordlist,
        word: String::from(secret_text),
        length: word_length((&secret_text).to_string()),
        rng : range_rand(game.words),
        active: true,
        created_at: SystemTime::now(),
    };
    word
}

fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}



// remove return type for real rng
fn range_rand(n: u64) -> u64 {
    rand::thread_rng().gen_range(0..n)
}


fn word_match(i: u64, n: u64) -> bool {
    if i == n {
        true
    } else {
        false
    }
}

fn init_word(game: &Game) {
    let num = range_rand(game.words);
    if let Ok(lines) = read_lines("./wordlist.txt") {
        let mut i: u64 = 0;
        for line in lines {
            if let Ok(text) = line {
                i = i + 1;
                if i == num {
                    let generated_word = new_word(&text, &game);
                    dbg!(generated_word);
                }
            }
        }
    }
}

// main game loop
fn main() {
    let wordlist = create_wordlist();
    let game = create_game(&wordlist);
    dbg!(&game);
    if let word = init_word(&game) {
        println!("Game Started!");
        println!("You have 15s to guess the word!");
    };
}