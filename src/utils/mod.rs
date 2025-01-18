use std::fs::File;
use std::io::Read;
use std::path::Path;

use std::collections::{HashMap, HashSet};

pub fn read_file_utf8(path: &Path) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn get_unique_characters(contents: &str) -> String {
    // get unique chareacters of it
    let unique_chars: HashSet<char> = contents.chars().collect();
    // println!("Number of unique characters: {}", unique_chars.len());

    // print out unique_chars, but try to sort it first
    let mut sorted_unique_chars: Vec<char> =
        unique_chars.iter().cloned().collect();
    sorted_unique_chars.sort();
    // println!("Unique characters: {:?}", sorted_unique_chars);
    // for i in sorted_unique_chars {
    //     print!("{i}");
    // }

    // try to not use print when return
    sorted_unique_chars.into_iter().collect()
}

pub fn create_mappings(
    chars: &str,
) -> (HashMap<char, usize>, HashMap<usize, char>) {
    let mut stoi = HashMap::new(); // Map from characters to integers
    let mut itos = HashMap::new(); // Map from integers to characters

    for (i, ch) in chars.chars().enumerate() {
        stoi.insert(ch, i);
        itos.insert(i, ch);
    }

    (stoi, itos)
}

pub fn encode(s: &str, stoi: &HashMap<char, usize>) -> Vec<usize> {
    s.chars().map(|c| *stoi.get(&c).unwrap()).collect()
}

pub fn decode(l: &[usize], itos: &HashMap<usize, char>) -> String {
    l.iter().map(|&i| *itos.get(&i).unwrap()).collect()
}
