use std::collections::HashSet;

use crate::util::generate_edge_repr;


// Type inference lets us omit an explicit type signature (which
// would be `HashSet<String>` in this example).

#[derive(Debug, Clone)]
pub(crate) struct Word {
    pub word: String,
    edges:	HashSet<String>,

}

impl Word {
    pub fn new(word: String) -> Self {

        let mut prev_letter = '_';
        let mut edges = HashSet::new();
        for letter in word.chars(){
            edges.insert(generate_edge_repr(prev_letter, letter));
            prev_letter = letter;

        } 

        edges.insert(generate_edge_repr(prev_letter, '_'));


        Word {	
            word,
            edges,
        }
    }


    pub fn has_subset_edges(&self, allowed_edges: &HashSet<String>) -> bool{

        for edge in self.edges.iter(){
            if !allowed_edges.contains(edge){
                return false
            }
        }

        return true
    }

}
