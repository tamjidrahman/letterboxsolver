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