use std::collections::HashSet;

use crate::letterboxsolver::LetterBoxSolver;

#[derive(Debug, Clone)]
pub(crate) struct Word {
    pub word: String,
    edges: HashSet<String>,
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
