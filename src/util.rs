use std::{fs::File, io::copy, path::Path};


pub fn download_file_if_not_exists(url: &str, path: &str){
    if !Path::new(path).exists() {
        println!(
            "-------Downloading file from {} to {}-------",
            url, path
        );
        let mut resp = reqwest::blocking::get(url).unwrap();

        let mut file = File::create(path).unwrap();

        copy(&mut resp, &mut file).unwrap();
    }

}

pub fn read_csv(filename: &str) -> Vec<String> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(filename)
        .unwrap();

    let records: Vec<String> = rdr
        .records()
        .into_iter()
        .map(|x| x.unwrap().get(0).unwrap().to_lowercase().to_string())
        .collect();

    return records;
}
