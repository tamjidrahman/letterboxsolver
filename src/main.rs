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

    print!("-------Loading dictionary from {}-------\r", DICTIONARY_PATH);
    let dictionary: Vec<Word> = read_csv(DICTIONARY_PATH)
        .into_par_iter()
        .map(|w| Word::new(w.to_string()))
        .collect();

    println!("-------Loading dictionary from {} complete!-------", DICTIONARY_PATH);

    // Read in problem structure from user input
    let mut input: String;
    let mut sides: Vec<Vec<char>> = vec![];
    
    for i in 0..4 {
        print!("Enter letters on side {}. (e.g.: xyz) :", i+1);
        input = read!();
        sides.push(input.chars().collect())
    }
    println!("\n\nSolving {sides:?}");
    
    // Construct game and solve
    let solution = LetterBoxSolver::new(dictionary, sides).solve();

    // Print out solution
    let solution_words: Vec<String> = solution.iter().map(|w| w.word.to_string()).collect();
    println!("Solution: {:?}", solution_words);

}