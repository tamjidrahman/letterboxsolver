use std::collections::HashSet;

use rayon::prelude::*;

use crate::word::Word;

trait Solver {
    fn solve(&self);
}

pub(crate) struct LetterBoxSolver {
    dictionary: Vec<Word>,
    sides: Vec<Vec<char>>,
    allowed_edges: HashSet<String>,
}

impl LetterBoxSolver {
    pub fn new(dictionary: Vec<Word>, sides: Vec<Vec<char>>) -> LetterBoxSolver {
        let allowed_edges = LetterBoxSolver::get_allowed_edges(&sides);

        LetterBoxSolver {
            dictionary,
            sides,
            allowed_edges,
        }
    }

    fn get_allowed_edges(sides: &Vec<Vec<char>>) -> HashSet<String> {
        /* Generate allowed edges between letters from sides
         */
        let mut edges: HashSet<String> = HashSet::new();
        for (i1, side1) in sides.iter().enumerate() {
            for letter1 in side1 {
                edges.insert(LetterBoxSolver::generate_edge_repr(Some(*letter1), None));
                edges.insert(LetterBoxSolver::generate_edge_repr(None, Some(*letter1)));
            }

            for (i2, side2) in sides.iter().enumerate() {
                if i1 == i2 {
                    continue;
                }
                for letter1 in side1 {
                    for letter2 in side2 {
                        edges.insert(LetterBoxSolver::generate_edge_repr(
                            Some(*letter1),
                            Some(*letter2),
                        ));
                    }
                }
            }
        }

        return edges;
    }

    pub fn generate_edge_repr(letter1: Option<char>, letter2: Option<char>) -> String {
        // Store magic of storing an edge as {letter1}-{letter2}, and treating blanks as _

        let l1_repr = match letter1 {
            None => '_',
            Some(c) => c,
        };

        let l2_repr = match letter2 {
            None => '_',
            Some(c) => c,
        };

        return format!("{l1_repr}-{l2_repr}");
    }

    pub fn solve(&self) -> Vec<Word> {
        let mut letters_remaining: HashSet<char> = HashSet::new();
        for side in self.sides.iter() {
            letters_remaining.extend(side.iter().copied())
        }

        let mut states: Vec<(Vec<Word>, HashSet<char>, Option<char>)> =
            vec![(vec![], letters_remaining, None)];

        loop {
            let new_states = self.generate_next_stage(states);

            let complete = new_states
                .iter()
                .filter(|(_words_used, letters_remaining, _starting_letter)| {
                    letters_remaining.iter().count() == 0
                })
                .map(|(words_used, _letters_remaining, _starting_letter)| words_used)
                .nth(0);

            if complete.is_some() {
                return complete.unwrap().clone();
            }
            states = new_states;
        }
    }

    fn generate_next_stage(
        &self,
        states: Vec<(Vec<Word>, HashSet<char>, Option<char>)>,
    ) -> Vec<(Vec<Word>, HashSet<char>, Option<char>)> {
        /* A stage is a set of states. For each state, we want to apply generate_next_states_from_state, and flatten, to generate the next stage
         */
        let new_states: Vec<(Vec<Word>, HashSet<char>, Option<char>)> = states
            .into_par_iter()
            .map(|(words_used, letters_remaining, starting_letter)| {
                self.generate_next_states_from_state(
                    &words_used,
                    &letters_remaining,
                    starting_letter,
                )
            })
            .flat_map(|s| s)
            .collect();

        return new_states;
    }

    fn generate_next_states_from_state(
        &self,
        words_used: &Vec<Word>,
        letters_remaining: &HashSet<char>,
        starting_letter: Option<char>,
    ) -> Vec<(Vec<Word>, HashSet<char>, Option<char>)> {
        /* One state can create many more states (i.e. there are many possible next words we can choose)
         */
        let mut new_states: Vec<(Vec<Word>, HashSet<char>, Option<char>)> = vec![];

        let allowed_next_words = self
            .dictionary
            .iter()
            .filter(|w| {
                (**w).has_subset_edges(&self.allowed_edges)
                    && (starting_letter.is_none() || (**w).word.chars().nth(0) == starting_letter)
            })
            .map(|w| (*w).clone());

        for word in allowed_next_words {
            let mut letters_remaining_new = letters_remaining.clone();

            for letter in word.word.chars() {
                letters_remaining_new.remove(&letter);
            }

            let starting_letter_new = word.word.chars().nth_back(0);
            let mut words_used_new = words_used.clone();
            words_used_new.push(word);

            new_states.push((words_used_new, letters_remaining_new, starting_letter_new));
        }

        let mut new_states_filtered: Vec<(Vec<Word>, HashSet<char>, Option<char>)> = vec![];

        for new_state1 in new_states.iter() {
            let mut is_redundant: bool = false;
            for new_state2 in new_states.iter() {
                // Skip pruning if the last character is different in each state
                if new_state1.2 != new_state2.2 {
                    continue;
                }
                // Prune if new_state2 letters remaining is a strict superset of new_state1 letters remaining
                else if new_state1.1.is_superset(&new_state2.1)
                    && !(new_state1.1.is_subset(&new_state2.1))
                {
                    is_redundant = true;
                    break;
                }
            }

            if !is_redundant {
                new_states_filtered.push(new_state1.clone());
            }
        }

        return new_states_filtered;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::word::Word;

    use super::LetterBoxSolver;

    fn generate_word_sequences_from_stage(
        stage: &Vec<(Vec<Word>, HashSet<char>, Option<char>)>,
    ) -> Vec<Vec<String>> {
        return stage
            .clone()
            .into_iter()
            .map(generate_word_sequence_from_state)
            .collect();
    }

    fn generate_word_sequence_from_state(
        state: (Vec<Word>, HashSet<char>, Option<char>),
    ) -> Vec<String> {
        let (word, _letters, _character) = state;
        return word.iter().map(|w| w.word.to_string()).collect();
    }

    #[test]
    fn test_generate_next_states_from_state() {
        let dict1: Vec<Word> = vec!["novy", "novelty", "naively", "foundation"]
            .iter()
            .map(|&s| Word::new(s.to_string()))
            .collect();
        let sides1: Vec<Vec<char>> = vec![
            vec!['y', 'd', 'e'],
            vec!['o', 'l', 'a'],
            vec!['f', 'i', 'n'],
            vec!['u', 'v', 't'],
        ];
        let solver1: LetterBoxSolver = LetterBoxSolver::new(dict1, sides1);

        let mut letters_remaining: HashSet<char> = HashSet::new();
        for side in solver1.sides.iter() {
            letters_remaining.extend(side.iter().copied())
        }

        let possible_first_states =
            solver1.generate_next_states_from_state(&vec![], &letters_remaining, None);
        assert_eq!(
            generate_word_sequences_from_stage(&possible_first_states),
            vec![vec!["novelty"], vec!["naively"], vec!["foundation"]]
        ); // foundation or novelty or naively are valid first answers. novy gets pruned by novelty

        let dead_end_state = solver1.generate_next_states_from_state(
            &vec![Word::new("novelty".to_string())],
            &letters_remaining,
            Some('y'),
        );
        let dead_state: Vec<Vec<String>> = vec![];
        assert_eq!(
            generate_word_sequences_from_stage(&dead_end_state),
            dead_state
        ); // starting with a y leads to no possible steps

        let penultimate_state = solver1.generate_next_states_from_state(
            &vec![Word::new("foundation".to_string())],
            &letters_remaining,
            Some('n'),
        );
        assert_eq!(
            generate_word_sequences_from_stage(&penultimate_state),
            vec![vec!["foundation", "novelty"], vec!["foundation", "naively"]]
        ); // two possible answers, both which are equivalent. no tiebreak defined to prune one or other
    }

    #[test]
    fn test_generate_next_stage() {
        let dict1: Vec<Word> = vec!["dog", "cate", "catfish", "dogc"]
            .iter()
            .map(|&s| Word::new(s.to_string()))
            .collect();
        let sides1: Vec<Vec<char>> = vec![
            vec!['d', 'a', 's'],
            vec!['o', 't', 'h'],
            vec!['g', 'e', 'i'],
            vec!['c', 'f', 'b'],
        ];
        let solver1: LetterBoxSolver = LetterBoxSolver::new(dict1, sides1);

        let mut letters_remaining: HashSet<char> = HashSet::new();
        for side in solver1.sides.iter() {
            letters_remaining.extend(side.iter().copied())
        }

        let states: Vec<(Vec<Word>, HashSet<char>, Option<char>)> =
            vec![(vec![], letters_remaining, None)];

        let first_stage = solver1.generate_next_stage(states);
        assert_eq!(
            generate_word_sequences_from_stage(&first_stage),
            vec![vec!["dog"], vec!["cate"], vec!["catfish"], vec!["dogc"]]
        );

        let second_stage: Vec<(Vec<Word>, HashSet<char>, Option<char>)> =
            solver1.generate_next_stage(first_stage);
        assert_eq!(
            generate_word_sequences_from_stage(&second_stage),
            vec![vec!["dogc", "cate"], vec!["dogc", "catfish"]]
        );
    }
}
