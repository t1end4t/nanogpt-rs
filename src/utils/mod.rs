use candle_core::IndexOp;
use candle_core::{Device::Cpu, Tensor};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::Path;

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

pub fn get_batch(
    split: &str,
    block_size: usize,
    batch_size: usize,
    train_data: Tensor,
    val_data: Tensor,
) -> (Tensor, Tensor) {
    let data = if split == "train" {
        train_data
    } else {
        val_data
    };

    // NOTE: prevent use it
    let len_data = data.shape().dim(0).unwrap();

    // NOTE: prevent use it
    let ix =
        Tensor::rand(0f32, (len_data - block_size) as f32, (batch_size,), &Cpu)
            .unwrap()
            .to_vec1::<f32>()
            .unwrap();

    // NOTE: prevent use it
    let x_slices: Vec<_> = ix
        .iter()
        .map(|&i| {
            data.i(i as usize..i as usize + block_size as usize)
                .unwrap()
        })
        .collect();
    let x = Tensor::stack(&x_slices, 0).unwrap();

    let y_slices: Vec<_> = ix
        .iter()
        .map(|&i| {
            data.i((i + 1f32) as usize..(i + 1f32) as usize + block_size)
                .unwrap()
        })
        .collect();

    let y = Tensor::stack(&y_slices, 0).unwrap();

    (x, y)
}
