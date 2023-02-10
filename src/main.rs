use args::{ResponseType, ScrambledArgs};
use clap::Parser;
use hashbrown::HashMap;
use std::cmp::Ordering;
use std::io::{BufRead, BufReader, Write};
use std::fs::File;

mod args;

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

fn check_singular_word(master_hash: &LetterTally, file: BufReader<File>) -> String {
    let scramble_len = master_hash.len();
    let mut longest = String::new();  
    let mut longest_len = 0;
    for line in file.lines() {
        let trial_word = line.unwrap();
        let word_len = trial_word.len();
        
        // dont bother checking words shorter than the longest or longer than the scramble
        if scramble_len < word_len || word_len < longest_len {
            continue;
        }
        if !validate_word(master_hash, &trial_word) {
            continue;
        }
        
        // update the new longest word
        // if it equals the scramble then we have a max length
        longest = trial_word.clone();
        longest_len = longest.len();
        if longest_len == scramble_len {
            break;
        }
    }
    longest
}

fn get_multiple_words(
    master_hash: &LetterTally, 
    max_count: Option<i32>, 
    min_length: Option<i32>, 
    file: BufReader<File>
) -> Vec<String> {
    let mut all_valid_words = Vec::new();
    let scramble_len = master_hash.len();

    let minimum = min_length.unwrap_or(0);

    for line in file.lines() {
        let trial_word = line.unwrap();
        let word_len = trial_word.len();
        
        // dont bother checking words shorter than the longest or longer than the scramble
        if scramble_len < word_len || scramble_len < minimum as _{
            continue;
        }
        if !validate_word(master_hash, &trial_word) {
            continue;
        }
        
        // update the new longest word
        // if it equals the scramble then we have a max length
        all_valid_words.push(trial_word);
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
    all_valid_words[..max_count.unwrap_or(all_valid_words.len() as _) as _].to_vec()
}

fn main() {
    // goal is to handle all of the arguments passed in here
    let args = ScrambledArgs::parse();

    // these variables are used in all cases
    let input_hash = convert_to_hash(&args.scramble); 
    let word_file = BufReader::new(File::open("words.txt").unwrap());
    
    // if none is provided then we want to print out singular word
    if let Some(response) = args.response_type {
        match response {
            ResponseType::File(f) => {
                // handles all of the max and mins already
                let all_valid_words = get_multiple_words(&input_hash, f.max_output, f.min_length, word_file);
                // replaces everything in the file
                let mut file = File::create(f.file_path).unwrap();
                for word in &all_valid_words {
                    writeln!(file, "{}", word).unwrap();
                }
            }
            ResponseType::List(l) => {
                // handles all of the max and mins already
                let all_valid_words = get_multiple_words(&input_hash, l.max_output, l.min_length, word_file);
                for word in &all_valid_words {
                    println!("{}", word);
                }
            }
        }
    } else {
        let longest_word = check_singular_word(&input_hash, word_file);
        println!("{}", longest_word);
    }
}
