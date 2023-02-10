use hashbrown::HashMap;
use std::cmp::Ordering;
use std::env;
use std::io::{BufRead, BufReader, Write};
use std::fs::File;

// contains each letter found in the word and how many times it shows up
type LetterTally = HashMap<char, i8>;

fn convert_to_hash(val: &str) -> LetterTally {
    let mut letters_hash = LetterTally::new();
    for c in val.chars().into_iter() {
        // if it isnt found adds tally of one
        *letters_hash.entry(c).or_insert(0) += 1;
    }
    letters_hash
}

fn validate_word(master_hash: &LetterTally, check_word: &str) -> bool {
    let mut check_hash = LetterTally::new();
    for c in check_word.chars().into_iter() {
        // if the word contains a letter
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
    let mut scramble = env::args().nth(1).expect("scramble not given");
    scramble = scramble.trim().to_string();
    
    // just to disregard all the words longer than the scramble
    let scramble_len = scramble.len();
    let input_hash = convert_to_hash(&scramble);
    let file = BufReader::new(File::open("words.txt").unwrap());

    let mut longest = String::new();  
    let mut all_valid_words = Vec::new();

    for line in file.lines() {
        let trial_word = line.unwrap();
        let word_len = trial_word.len();
        
        // dont bother checking words shorter than the longest or longer than the scramble
        if scramble_len < word_len {
            continue;
        }
        if !validate_word(&input_hash, &trial_word) {
            continue;
        }
        
        // update the new longest word
        // if it equals the scramble then we have a max length
        longest = trial_word.clone();
        all_valid_words.push(trial_word);
        if longest.len() == scramble_len {
            break;
        }
    }
    all_valid_words.sort_by(|a, b| {
        if a.len() > b.len() {
            Ordering::Less
        }
        else if a.len() == b.len() {
            Ordering::Equal
        }
        else {
            Ordering::Greater
        }   
    });

    if let Some(response) = env::args().nth(2) {
        match response.as_str() {
            "--list" => {
                for word in all_valid_words {
                    println!("{}", word);
                }
            }
            "--word" => {
                println!("{}", longest);
            }
            "--file" => {
                let file_path = env::args().nth(3).expect("file path not given");
                let mut file = File::create(file_path).unwrap();
                for word in all_valid_words {
                    writeln!(file, "{}", word).unwrap();
                }
            }
            _ => {
                panic!("invalid response type");
            }
        }
    } else {
        println!("{}", longest);
    }
}
