use core::str;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;
use std::{
    char,
    collections::{BTreeSet, HashMap},
};

//NOTE: we don't need to add backspace like python
pub fn find_unique_char(s: &str) -> String {
    let mut set: BTreeSet<_> = BTreeSet::new();

    for ch in s.chars() {
        set.insert(ch);
    }

    let vec_ch = set.into_iter().collect::<Vec<_>>();
    let out: String = vec_ch.iter().collect();

    out
}

// NOTE: remember that is reverse version of it in python
pub fn encode(s: &str, sample: &str) -> Vec<usize> {
    let mut encoded_result: Vec<usize> = Vec::new();

    let mut hash = HashMap::new();
    for (idx, ch) in sample.chars().enumerate() {
        hash.insert(ch, idx);
    }

    for ch in s.chars() {
        match hash.get(&ch) {
            Some(&idx) => encoded_result.push(idx),
            None => println!("Can not contain character from sample"),
        }
    }

    encoded_result
}

pub fn decode(vec: &Vec<usize>, sample: &str) -> Vec<char> {
    let mut decode_result: Vec<char> = Vec::new();

    // reverse
    let mut hash = HashMap::new();
    for (idx, ch) in sample.chars().enumerate() {
        hash.insert(idx, ch);
    }

    for idx in vec {
        match hash.get(&idx) {
            Some(&ch) => decode_result.push(ch),
            None => println!("Can not contain character from sample"),
        }
    }

    decode_result
}

// TODO: try to random with seed first
// pub fn random_with_seed(seed: f32) {
//     let mut rng = Pcg64::seed_from_u64(42);
//     println!("Random number: {}", rng.random());
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_unique_char() {
        let input = String::from("hello");
        let result = find_unique_char(&input);
        assert_eq!(result, "afsd");
    }
}
