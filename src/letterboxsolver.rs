use std::collections::HashSet;

use rayon::prelude::*;

use crate::word::Word;
use crate::util::get_allowed_edges;

trait Solver {
    fn solve(&self);
}

pub(crate) struct LetterBoxSolver {
    words: Vec<Word>,
    sides: Vec<Vec<char>>,
    allowed_edges: HashSet<String>
}

impl LetterBoxSolver {

    pub fn new(words: Vec<Word>, sides: Vec<Vec<char>>) -> LetterBoxSolver{

        let allowed_edges = get_allowed_edges(&sides);

        LetterBoxSolver {
            words,
            sides,
            allowed_edges,
        }

    }

    pub fn solve(&self){
        
        // println!("{:?}", allowed_edges.len());

        let mut letters_remaining: HashSet<char> = HashSet::new();
        for side in self.sides.iter(){
            letters_remaining.extend(side.iter().copied())
        }

        let mut states: Vec<(Vec<Word>, HashSet<char>, Option<char>)> = vec![(vec![], letters_remaining, None)];


        loop{

            let new_states = self.update_states(states);

            let complete = new_states.iter()
            .filter(|(_words_used, letters_remaining, _starting_letter)| letters_remaining.iter().count() == 0)
            .map(|(words_used, _letters_remaining, _starting_letter)| words_used)
            .nth(0);

            if complete.is_some(){
                println!("{:?}", complete.unwrap());
                break
            }
            states = new_states;

        }
    }

    fn update_states(&self, states: Vec<(Vec<Word>, HashSet<char>, Option<char>)>) -> Vec<(Vec<Word>, HashSet<char>, Option<char>)>{
        
        let new_states: Vec<(Vec<Word>, HashSet<char>, Option<char>)> = states.into_par_iter()
        .map(|(words_used, letters_remaining, starting_letter)| self.solve_step(&words_used, &letters_remaining, starting_letter))
        .flat_map(|s| s)
        .collect();

        return new_states;
    }

    fn solve_step(&self, words_used: &Vec<Word>, letters_remaining: &HashSet<char>, starting_letter: Option<char>) -> Vec<(Vec<Word>, HashSet<char>, Option<char>)>{

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