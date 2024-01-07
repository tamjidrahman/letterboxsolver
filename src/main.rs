mod util;
mod word;
mod letterboxsolver;
use text_io::read;


use rayon::prelude::*;
use util::read_csv;
use word::Word;
use letterboxsolver::LetterBoxSolver;

const DICTIONARY_PATH: &str = "words.txt";

fn main() {
    let words: Vec<Word> = read_csv(DICTIONARY_PATH)
        .into_par_iter()
        .map(|w| Word::new(w.to_string()))
        .collect();

    let mut input: String;

    let mut sides: Vec<Vec<char>> = vec![
    ];

    println!("Enter letters on each sides, separated by a new line");
    for _ in 0..4 {
        input = read!();
        sides.push(input.chars().collect())
    }
    println!("Solving {sides:?}");
    
    let game = LetterBoxSolver::new(words, sides);
    game.solve()
}