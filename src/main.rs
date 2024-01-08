mod letterboxsolver;
mod util;
mod word;

use letterboxsolver::LetterBoxSolver;
use word::load_words_from_csv;

use crate::util::download_file_if_not_exists;

slint::include_modules!();
const DICTIONARY_PATH: &str = "dictionary.txt";
const DICTIONARY_URL: &str =
    "https://raw.githubusercontent.com/tamjidrahman/letterboxsolver/main/words.txt";

fn main() -> Result<(), slint::PlatformError> {
    // if word.txt doesn't exist, download from github
    download_file_if_not_exists(DICTIONARY_URL, DICTIONARY_PATH);

    // load dictionary
    let dictionary = load_words_from_csv(DICTIONARY_PATH);

    let ui = AppWindow::new()?;

    ui.on_solve(move |side1, side2, side3, side4| {
        let sides: Vec<Vec<char>> = vec![side1, side2, side3, side4]
            .iter()
            .map(|side| side.chars().collect())
            .collect();
        let solution = LetterBoxSolver::new(dictionary.clone(), sides).solve();
        let solution_words: Vec<String> = solution.iter().map(|w| w.word.to_string()).collect();
        let ui_solution: String = solution_words.iter().fold("".to_string(), |acc, s| {
            if acc != "" {
                format!("{}, {}", acc, s)
            } else {
                s.to_string()
            }
        });
        return ui_solution.into();
    });

    ui.run()
}
