use std::collections::HashSet;

use rayon::prelude::*;

use crate::word::Word;

trait Solver {
    fn solve(&self);
}

pub(crate) struct LetterBoxSolver {
    words: Vec<Word>,
    sides: Vec<Vec<char>>,
    allowed_edges: HashSet<String>
}

impl LetterBoxSolver {

    pub fn new(dictionary: Vec<Word>, sides: Vec<Vec<char>>) -> LetterBoxSolver{

        let allowed_edges = LetterBoxSolver::get_allowed_edges(&sides);

        LetterBoxSolver {
            words: dictionary,
            sides,
            allowed_edges,
        }

    }

    fn get_allowed_edges(sides: &Vec<Vec<char>>) -> HashSet<String>{
        /* Generate allowed edges between letters from sides
         */
        let mut edges: HashSet<String> = HashSet::new();
        for (i1, side1) in sides.iter().enumerate(){
    
            for letter1 in side1{
                edges.insert(LetterBoxSolver::generate_edge_repr(Some(*letter1), None));
                edges.insert(LetterBoxSolver::generate_edge_repr(None, Some(*letter1)));
            }
    
            for (i2, side2) in sides.iter().enumerate(){
                if i1 == i2{
                    continue;
                }
                for letter1 in side1{
                    for letter2 in side2{
                        edges.insert(LetterBoxSolver::generate_edge_repr(Some(*letter1), Some(*letter2)));
                    }
    
                }
            }
        }
    
        return edges
    }

    pub fn generate_edge_repr(letter1: Option<char>, letter2: Option<char>) -> String{
        // Store magic of storing an edge as {letter1}-{letter2}, and treating blanks as _

        let l1_repr = match letter1 {
            None => '_',
            Some(c) => c,
        };

        let l2_repr = match letter2 {
            None => '_',
            Some(c) => c,
        };
        
        return format!("{l1_repr}-{l2_repr}")
    }



    pub fn solve(&self) -> Vec<Word>{
        
        let mut letters_remaining: HashSet<char> = HashSet::new();
        for side in self.sides.iter(){
            letters_remaining.extend(side.iter().copied())
        }

        let mut states: Vec<(Vec<Word>, HashSet<char>, Option<char>)> = vec![(vec![], letters_remaining, None)];


        loop{

            let new_states = self.generate_next_stage(states);

            let complete = new_states.iter()
            .filter(|(_words_used, letters_remaining, _starting_letter)| letters_remaining.iter().count() == 0)
            .map(|(words_used, _letters_remaining, _starting_letter)| words_used)
            .nth(0);

            if complete.is_some(){
                return complete.unwrap().clone()
            }
            states = new_states;

        }
    }

    fn generate_next_stage(&self, states: Vec<(Vec<Word>, HashSet<char>, Option<char>)>) -> Vec<(Vec<Word>, HashSet<char>, Option<char>)>{
        /* A stage is a set of states. For each state, we want to apply generate_next_states_from_state, and flatten, to generate the next stage
         */
        let new_states: Vec<(Vec<Word>, HashSet<char>, Option<char>)> = states.into_par_iter()
        .map(|(words_used, letters_remaining, starting_letter)| self.generate_next_states_from_state(&words_used, &letters_remaining, starting_letter))
        .flat_map(|s| s)
        .collect();

        return new_states;
    }

    fn generate_next_states_from_state(&self, words_used: &Vec<Word>, letters_remaining: &HashSet<char>, starting_letter: Option<char>) -> Vec<(Vec<Word>, HashSet<char>, Option<char>)>{
        /* One state can create many more states (i.e. there are many possible next words we can choose)
         */
        let mut new_states: Vec<(Vec<Word>, HashSet<char>, Option<char>)> = vec![(vec![], letters_remaining.clone(), None)];
        
        let allowed_next_words = self.words
            .iter()
            .filter(|w| (**w).has_subset_edges(&self.allowed_edges) && (starting_letter.is_none() || (**w).word.chars().nth(0) == starting_letter))
            .map(|w| (*w).clone());

        for word in allowed_next_words{

            let mut letters_remaining_new = letters_remaining.clone();
    
            for letter in word.word.chars(){
                letters_remaining_new.remove(&letter);
            }

            let starting_letter_new =  word.word.chars().nth_back(0);
            let mut words_used_new = words_used.clone();
            words_used_new.push(word);

            new_states.push((
                words_used_new,
                letters_remaining_new,
                starting_letter_new
            ));
        }

        let mut new_states_filtered: Vec<(Vec<Word>, HashSet<char>, Option<char>)> = vec![];

        for new_state1 in new_states.iter(){
            let mut is_redundant:bool = false;
            for new_state2 in new_states.iter(){
                if new_state1.2 != new_state2.2 || new_state1.0.iter().nth_back(0).unwrap_or(&Word::new("".to_string())).word == new_state2.0.iter().nth_back(0).unwrap_or(&Word::new("".to_string())).word {
                    continue;
                }

                else if new_state1.1.is_superset(&new_state2.1){
                    is_redundant = true;
                }
            }

            if !is_redundant{
                new_states_filtered.push(new_state1.clone());
            }
        }

        return new_states_filtered;

    }
}