use hashbrown::HashMap;
use std::io::{BufRead, BufReader, stdin, stdout, Write};
use std::fs::File;

type LetterTally = HashMap<char, i8>;

fn convert_to_hash(val: &str) -> LetterTally {
    let mut letters_hash = LetterTally::new();
    for c in val.chars().into_iter() {
        *letters_hash.entry(c).or_insert(0) += 1;
    }
    letters_hash
}

fn validate_word(master_hash: &LetterTally, check_word: &str) -> bool {
    let mut check_hash = LetterTally::new();
    for c in check_word.chars().into_iter() {
        if !master_hash.contains_key(&c) {
            return false
        }
        *check_hash.entry(c).or_insert(0) += 1;
        if check_hash.get(&c).unwrap() > master_hash.get(&c).unwrap() {
            return false
        }
    }
    true
}

fn main() {
    print!("what is the scramble: ");
    stdout().flush().unwrap();

    let mut scramble = String::new();
    stdin().read_line(&mut scramble).unwrap();
    scramble = scramble.trim().to_string();

    let scramble_len = scramble.len();
    let input_hash = convert_to_hash(&scramble);
    let file = BufReader::new(File::open("words.txt").unwrap());

    let mut longest = String::new();  
    let mut len_longest = 0;  
    for line in file.lines() {
        let trial_word = line.unwrap();
        let word_len = trial_word.len();
        if scramble_len < word_len || word_len < len_longest {
            continue;
        }
        if !validate_word(&input_hash, &trial_word) {
            continue;
        }
        longest = trial_word;
        len_longest = longest.len();
        if len_longest == scramble_len {
            break;
        }
    }
    println!("the longest word is: {}", longest);
    stdin().read_line(&mut String::new()).unwrap();
}
