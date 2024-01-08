use std::collections::HashSet;

use rayon::prelude::*;

use crate::{letterboxsolver::LetterBoxSolver, util::read_csv};

#[derive(Debug, Clone)]
pub(crate) struct Word {
    pub word: String,
    edges: HashSet<String>,
}

pub fn load_words_from_url(url: &str) -> Vec<Word>{
    
    let resp = reqwest::blocking::get(url).unwrap();
    let mut rdr = csv::Reader::from_reader(resp);
    let words = rdr.records().map(|row| row.expect("failed to load").get(0).expect("a").to_string() ).map(|s| Word::new(s)).collect();
    
    return words

}

pub fn load_words_from_csv(filename: &str) -> Vec<Word>{
    let dictionary: Vec<Word> = read_csv(filename)
        .into_par_iter()
        .map(|w| Word::new(w.to_string()))
        .collect();

    println!(
        "-------Loading dictionary from {} complete!-------",
        filename
    );

    return dictionary;
}

impl Word {
    pub fn new(word: String) -> Self {
        let mut prev_letter: Option<char> = None;
        let mut edges = HashSet::new();
        for letter in word.chars() {
            edges.insert(LetterBoxSolver::generate_edge_repr(
                prev_letter,
                Some(letter),
            ));
            prev_letter = Some(letter);
        }

        edges.insert(LetterBoxSolver::generate_edge_repr(prev_letter, None));

        Word { word, edges }
    }

    pub fn has_subset_edges(&self, allowed_edges: &HashSet<String>) -> bool {
        for edge in self.edges.iter() {
            if !allowed_edges.contains(edge) {
                return false;
            }
        }

        return true;
    }
}
