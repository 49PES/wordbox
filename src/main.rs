use std::fs::File;
use std::io::{BufRead, BufReader};

use rand::prelude::*;
use rand::seq::SliceRandom;

// use itertools::Itertools;

fn filter_words(filename: &str) -> Vec<String> {
    let file: File = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map_while(Result::ok)
        .filter(|line| {
            line.chars()
                .all(|c| !c.is_uppercase() && !c.is_ascii_punctuation() && !c.is_whitespace())
        })
        .collect()
}

fn pick_random_strings<'a>(strings: &'a [&'a str]) -> Vec<&'a str> {
    let mut rng = thread_rng();
    strings.choose_multiple(&mut rng, 3).copied().collect()
}
/*
fn is_word_box(words: &[&String]) -> bool {
    if words.len() != 3 {
        return false;
    }

    for (i, word1) in words.iter().enumerate() {
        for (j, word2) in words.iter().enumerate().skip(i + 1) {
            if word1.len() != 3 || word2.len() != 3 {
                continue;
            }

            let mut is_word_box = true;

            for k in 0..3 {
                let mut row = [false; 3];
                let mut col = [false; 3];

                for l in 0..3 {
                    if word1.chars().nth(l) == word2.chars().nth(k) {
                        row[l] = true;
                        col[k] = true;
                    }
                }

                if !row.iter().all(|&x| x) || !col.iter().all(|&x| x) {
                    is_word_box = false;
                    break;
                }
            }

            if is_word_box {
                return true;
            }

            if i != j {
                let mut row = [false; 3];
                let mut col = [false; 3];

                for k in 0..3 {
                    if word1.chars().nth(k) == word2.chars().nth(3 - k) {
                        row[k] = true;
                        col[3 - k] = true;
                    }
                }

                if !row.iter().all(|&x| x) || !col.iter().all(|&x| x) {
                    is_word_box = false;
                    break;
                }

                if is_word_box {
                    return true;
                }
            }
        }
    }

    false
}
*/

/*
fn is_word_box(words: &[&String]) -> bool {
    if words.len() != 3 {
        return false;
    }

    for i in 0..3 {
        let word1 = words[i];
        for j in (i + 1)..3 {
            let word2 = words[j];

            if word1.len() != 3 || word2.len() != 3 {
                continue;
            }

            let mut is_word_box = true;

            for k in 0..3 {
                let mut row = [false; 3];
                let mut col = [false; 3];

                for l in 0..3 {
                    if word1.chars().nth(l) == word2.chars().nth(k) {
                        row[l] = true;
                        col[k] = true;
                    }
                }

                if !row.iter().all(|&x| x) || !col.iter().all(|&x| x) {
                    is_word_box = false;
                    break;
                }
            }

            if is_word_box {
                return true;
            }
        }
    }

    false
}
*/

fn is_word_box(words: &[&str]) -> bool {
    // Check if there are exactly 3 words and each word is exactly 3 characters long
    if words.len() != 3 || !words.iter().all(|word| word.len() == 3) {
        return false;
    }

    // Check if the grid is symmetric
    for i in 0..3 {
        for j in 0..3 {
            if words[i].chars().nth(j) != words[j].chars().nth(i) {
                return false;
            }
        }
    }

    true
}

fn main() {
    let words = filter_words("3esl.txt");
    let four_letter_words: Vec<_> = words
        .iter()
        .filter(|line| line.len() == 3)
        .map(|s| s.as_str())
        .collect();

    // let valid: Vec<String> = vec!["mill".to_string(), "idea".to_string(), "lead".to_string(), "mend".to_string()];
    // let valid_word_refs: Vec<&String> = valid.iter().collect();

    // println!("{:#?}, {}", &valid, is_word_box(&valid_word_refs));

    let mut random_words = pick_random_strings(four_letter_words.as_slice());
    while !is_word_box(&random_words) {
        random_words = pick_random_strings(four_letter_words.as_slice());
    }

    println!("{:#?}", random_words);
}