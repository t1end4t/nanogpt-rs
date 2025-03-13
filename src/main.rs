use nanogpt_rs::decode;
use nanogpt_rs::encode;
use nanogpt_rs::find_unique_char;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let file_contents = fs::read_to_string("input.txt")?;

    // Let's look at the first 1000 characters
    println!("===========================================");
    println!("{}", &file_contents[..1000]);

    // get unique characters, including backspace
    println!("===========================================");
    let chars = find_unique_char(&file_contents);
    let vocab_size = chars.chars().count();
    println!("{chars}");
    println!("Vocab size: {vocab_size}");

    // Create a mapping from character to integers
    println!("===========================================");
    let encode_result = encode(&"hii there", &chars);
    let decode_result = decode(&encode_result, &chars);

    println!("{:?}", encode_result);
    println!("{:?}", decode_result);

    // try encode all contents
    let data = encode(&file_contents, &chars);
    println!("{:?}", &data[..1000]);

    // Split into train and valid sets (80, 10)
    // use block_size = 8 go though each elements
    println!("===========================================");
    let split_idx = (0.9f32 * data.len() as f32) as usize;

    let train_data = &data[..split_idx];
    // let val_data = &data[split_idx..];

    let block_size: usize = 8;

    let x = &train_data[..block_size];
    let y = &train_data[1..block_size + 1];

    for idx in 0..block_size {
        let content = &x[..idx + 1];
        let target = &y[idx];

        println!(
            "when input is tensor {:?} the targer: {:?}",
            content, target
        );
    }

    // We try to random seed
    // batch size set to 4 and block size is 8
    // get_batch function (input is "train" or "test" to choose train or val data)
    // this function random vector have size (batch, )
    // do the same like above
    println!("===========================================");

    Ok(())
}
