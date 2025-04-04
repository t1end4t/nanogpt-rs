use core::str;
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

/// TODO: we not random string
pub fn random_string_with_seed(length: usize, seed: u64) -> String {
    use rand::{Rng, SeedableRng};
    use rand_pcg::Pcg64;

    let mut rng = Pcg64::seed_from_u64(seed);
    (0..length)
        .map(|_| {
            // Generate a random ASCII character (32-126)
            rng.random_range(2..127) as u8 as char
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_unique_char() {
        let input = String::from("hello");
        let result = find_unique_char(&input);
        assert_eq!(result, "ehlo");
    }

    #[test]
    fn test_random_string_with_seed() {
        let seed = 12345;
        let length = 10;
        let result1 = random_string_with_seed(length, seed);
        let result2 = random_string_with_seed(length, seed);

        // Same seed should produce same result
        assert_eq!(result1, result2);

        // Different seed should produce different result
        let result3 = random_string_with_seed(length, seed + 1);
        assert_ne!(result1, result3);

        // Correct length
        assert_eq!(result1.len(), length);
    }
}
