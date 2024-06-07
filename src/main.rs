use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WordBox {
    row_dim: usize,    // number of rows
    col_dim: usize,    // number of columns
    rows: Vec<String>, // the words for each ROW
}

impl Display for WordBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid: Vec<Vec<char>> = vec![vec!['_'; self.col_dim]; self.row_dim];

        for (i, word) in self.rows.iter().enumerate() {
            for (j, ch) in word.chars().enumerate() {
                grid[i][j] = ch;
            }
        }

        for row in &grid {
            for ch in row {
                write!(f, "{} ", ch)?;
            }
            writeln!(f, "\n")?;
        }

        Ok(())
    }
}

pub trait Lexicon {
    // some constructor
    fn initialize(words: Vec<String>, lengths: Vec<usize>) -> Self;

    // whatever methods you have
    fn words_with_prefix(&self, prefix: &String, word_len: usize) -> Vec<String>;
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

    fn words_with_prefix(&self, prefix: &String, word_len: usize) -> Vec<String> {
        self.words
            .iter()
            .filter(|word| word.starts_with(prefix) && word.len() == word_len)
            .cloned()
            .collect()
    }
}

pub struct HashMapLexicon {
    words: HashMap<String, Vec<String>>,
}

impl Lexicon for HashMapLexicon {
    /// Get a list of words that start with the given prefix and are of the given length
    fn initialize(words: Vec<String>, lengths: Vec<usize>) -> Self {
        unimplemented!("")
    }

    fn words_with_prefix(&self, prefix: &String, word_len: usize) -> Vec<String> {
        unimplemented!("")
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
    fn is_done(&self) -> bool {
        self.rows.len() == self.row_dim
    }

    fn take_ith_characters(words: &[String], i: usize) -> String {
        words
            .iter()
            .map(|word| word.chars().nth(i).unwrap())
            .collect()
    }

    fn next_moves<L: Lexicon>(&self, vec_lexicon: &L) -> Vec<String> {
        let mut moves = vec![];
        let mut rows: Vec<String> = self.rows.clone();
        for word in vec_lexicon.words_with_prefix(&"".to_string(), self.col_dim) {
            rows.push(word.clone());
            let mut flag: bool = true;
            for i in 0..self.col_dim {
                let prefix = Self::take_ith_characters(&rows, i);
                let choice = vec_lexicon.words_with_prefix(&prefix, self.row_dim);
                if choice.is_empty() {
                    flag = false;
                    break;
                }
            }
            if flag {
                moves.push(word.clone());
            }
            rows.remove(rows.len() - 1);
        }
        moves
    }

    fn add_word(&self, word: String) -> WordBox {
        let mut rows: Vec<String> = self.rows.clone();
        rows.push(word.clone());
        WordBox {
            row_dim: self.row_dim,
            col_dim: self.col_dim,
            rows,
        }
    }
}

fn solve_word_box(wb: WordBox, vec_lexicon: &VecLexicon) -> Option<WordBox> {
    println!("{}", wb);
    if wb.is_done() {
        return Some(wb);
    }
    let choices = wb.next_moves(vec_lexicon);

    for choice in choices {
        let sol = solve_word_box(wb.add_word(choice), vec_lexicon);
        if sol.is_some() {
            return sol;
        }
    }
    None
}
fn main() {
    let words = filter_words("../3esl.txt");
    let vec_lexicon = VecLexicon { words };

    let rows: Vec<String> = vec![];
    let word_box = WordBox {
        row_dim: 6,
        col_dim: 6,
        rows,
    };

    println!("{}", vec_lexicon.words.len());
    let word_box_option = solve_word_box(word_box, &vec_lexicon);

    match word_box_option {
        Some(word_box) => println!("{}", word_box),
        None => println!("No word box found.\n"),
    }
}
