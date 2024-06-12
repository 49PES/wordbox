use crossterm::{
    cursor, execute,
    style::{self, Stylize},
    terminal::{self},
};
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::fmt::Display;
use std::fs::File;
use std::io::{stdout, BufRead, BufReader};
use std::time::Instant;
use std::{collections::HashMap, vec};
use tqdm::tqdm;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WordBox {
    row_dim: usize,    // number of rows
    col_dim: usize,    // number of columns
    rows: Vec<String>, // the words for each row
    cols: Vec<String>, // the words for each column
    is_symmetric: bool,
}

impl Display for WordBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid: Vec<Vec<char>> = vec![vec!['_'; self.col_dim]; self.row_dim];

        for (i, word) in self.rows.iter().enumerate() {
            for (j, ch) in word.chars().enumerate() {
                grid[i][j] = ch;
            }
        }

        for (i, word) in self.cols.iter().enumerate() {
            for (j, ch) in word.chars().enumerate() {
                grid[j][i] = ch;
            }
        }

        for row in &grid {
            for ch in row {
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
/*
impl Ord for WordBox {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // self.score().cmp(&other.score())
        unimplemented!()
    }
}
*/
pub trait Lexicon {
    fn initialize(words: Vec<String>, lengths: Vec<usize>) -> Self;

    fn words_with_prefix(&self, prefix: &str, word_len: usize) -> Vec<String>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VecLexicon {
    words: Vec<String>,
}

impl Lexicon for VecLexicon {
    /// Get a list of words that start with the given prefix and are of the given length
    fn initialize(words: Vec<String>, lengths: Vec<usize>) -> Self {
        VecLexicon {
            words: words
                .iter()
                .filter(|word| lengths.contains(&word.len()))
                .cloned()
                .collect(),
        }
    }

    fn words_with_prefix(&self, prefix: &str, word_len: usize) -> Vec<String> {
        self.words
            .iter()
            .filter(|word| word.starts_with(prefix) && word.len() == word_len)
            .cloned()
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashMapLexicon {
    words: HashMap<String, Vec<String>>,
}

impl Lexicon for HashMapLexicon {
    /// Get a list of words that start with the given prefix and are of the given length
    fn initialize(words: Vec<String>, lengths: Vec<usize>) -> Self {
        let mut words_map: HashMap<String, Vec<String>> = HashMap::new();
        for word in words.iter() {
            if lengths.contains(&word.len()) {
                for i in 0..=word.len() {
                    words_map
                        .entry(word[..i].to_string())
                        .or_default()
                        .push(word.clone());
                }
            }
        }
        HashMapLexicon { words: words_map }
    }

    fn words_with_prefix(&self, prefix: &str, word_len: usize) -> Vec<String> {
        self.words
            .get(prefix)
            .unwrap_or(&vec![])
            .iter()
            .filter(|w| w.len() == word_len)
            .cloned()
            .collect()
    }
}

impl Display for VecLexicon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.words.join(", "))
    }
}

/// Filter out words that contain uppercase letters, punctuation, or whitespace
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

impl WordBox {
    /*
    fn score(&self) -> f64 {
        let mut prod = 1.0;
        for i in 0..self.col_dim {
            let prefix = Self::take_ith_characters(&self.rows, i);
            let choices = lexicon.words_with_prefix(&prefix, self.row_dim);
            prod *= choices.len() as f64;
        }

        (100 * self.rows.len()) as f64 + prod
    }
    */

    fn is_done(&self) -> bool {
        self.rows.len() == self.row_dim
    }

    fn take_ith_characters(words: &[String], i: usize) -> String {
        words
            .iter()
            .map(|word| word.chars().nth(i).unwrap())
            .collect()
    }

    fn is_valid_move<L: Lexicon>(&self, word: &str, lexicon: &L) -> bool {
        let mut rows: Vec<String> = self.rows.clone();
        rows.push(word.to_string());
        for i in 0..self.col_dim {
            let prefix = Self::take_ith_characters(&rows, i);
            let choices = lexicon.words_with_prefix(&prefix, self.row_dim);
            if choices.is_empty() {
                return false;
            }
        }
        true
    }

    fn add_word(&self, word: String) -> WordBox {
        let mut rows: Vec<String> = self.rows.clone();
        rows.push(word.clone());
        let mut cols = self.cols.clone();
        if self.is_symmetric {
            cols.push(word.clone());
        }
        WordBox {
            row_dim: self.row_dim,
            col_dim: self.col_dim,
            rows,
            cols,
            is_symmetric: self.is_symmetric,
        }
    }
}

fn print_clear(wb: &WordBox) {
    execute!(
        stdout(),
        cursor::RestorePosition,
        style::PrintStyledContent(wb.to_string().cyan().bold())
    );
}

fn solve_word_box<L: Lexicon>(wb: WordBox, lexicon: &L) -> Option<WordBox> {
    let mut boxes: VecDeque<WordBox> = VecDeque::from([wb]);
    while !boxes.is_empty() {
        let wb = boxes.pop_front().unwrap();
        // execute!(stdout(), terminal::Clear(terminal::ClearType::All)).ok();
        // print_clear(&wb);
        if wb.is_done() {
            return Some(wb);
        }

        let prefix = WordBox::take_ith_characters(&wb.cols, wb.rows.len());
        let binding = lexicon.words_with_prefix(&prefix, wb.col_dim);
        let choices = binding
            .iter()
            .filter(|word| wb.is_valid_move(word, lexicon));

        for choice in choices {
            boxes.push_front(wb.add_word(choice.to_string()));
        }
    }
    None
}
fn main() {
    let start = Instant::now();
    let words = filter_words("../3esl.txt");

    // Find all word boxes of row_dim x col_dim
    let row_dim = 6;
    let col_dim = 6;

    let lexicon = HashMapLexicon::initialize(words, vec![row_dim, col_dim]);

    lexicon
        .words_with_prefix("", col_dim)
        .iter()
        .for_each(|word| {
            let word_box_option = solve_word_box(
                WordBox {
                    row_dim,
                    col_dim,
                    rows: vec![word.to_string()],
                    cols: vec![word.to_string()],
                    is_symmetric: true,
                },
                &lexicon,
            );

            match word_box_option {
                Some(word_box) => {
                    execute!(stdout(), terminal::Clear(terminal::ClearType::All)).ok();
                    print_clear(&word_box);
                    // println!("{}", word_box);
                }
                None => (),
            }
        });
    let duration = start.elapsed();
    println!("Time Duration: {:?}", duration);
}
