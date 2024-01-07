use std::collections::HashSet;

pub fn get_allowed_edges(sides: &Vec<Vec<char>>) -> HashSet<String>{
    
    let mut edges: HashSet<String> = HashSet::new();
    for (i1, side1) in sides.iter().enumerate(){

        for letter1 in side1{
            edges.insert(generate_edge_repr(*letter1, '_'));
            edges.insert(generate_edge_repr('_', *letter1));
        }

        for (i2, side2) in sides.iter().enumerate(){
            if i1 == i2{
                continue;
            }
            for letter1 in side1{
                for letter2 in side2{
                    edges.insert(generate_edge_repr(*letter1, *letter2));
                }

            }
        }
    }

    return edges
}

pub fn generate_edge_repr(letter1: char, letter2: char) -> String{
    return format!("{letter1}-{letter2}")
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