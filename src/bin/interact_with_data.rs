use nanogpt_rs::utils::{
    create_mappings, encode, get_unique_characters, read_file_utf8,
};

use candle_core::{Device::Cpu, IndexOp, Tensor};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = Path::new("./data/input.txt"); // Replace with your file path

    match read_file_utf8(file_path) {
        Err(error) => {
            eprintln!("Error reading file: {}", error);
        }

        Ok(contents) => {
            println!("length of dataset in characters: {}", contents.len());

            // print out first 1000 character of it
            // println!(
            //     "First 1000 characters: {}",
            //     // NOTE: that is good code!!!
            //     &contents[..std::cmp::min(1000, contents.len())]
            // );
            let chars = get_unique_characters(&contents);

            let (stoi, _) = create_mappings(&chars);

            // let input_string = "hii there";
            // let encoded = encode(input_string, &stoi);
            // println!("Encoded: {:?}", encoded);

            // let decoded = decode(&encoded, &itos);
            // println!("Decoded: {}", decoded);

            // try to use candle_core
            let encoded: Vec<f32> =
                encode(&contents, &stoi).iter().map(|&x| x as f32).collect();

            // NOTE: prevent use it
            let data = Tensor::new(encoded, &Cpu)?;
            // println!("Shape and type: {:?}, {:?}", data.shape(), data.dtype());
            // let slice_data = data.i(..100)?;
            // println!(
            //     "Get value of index 0: {:?}",
            //     slice_data.to_vec1::<f32>()?
            // );
            // println!("{:?}", data[1]);

            // split up the data into train and valid sets
            // len()
            let n =
                (0.9 * data.shape().dim(0).unwrap() as f32).round() as usize;
            println!("{:?}", n);

            let train_data = data.i(..n)?;
            let valid_data = data.i(n..)?;

            // let block_size = 8;
            // println!("{:?}", train_data.i(..block_size + 1));

            // sequence
            // let x = train_data.i(..block_size)?;
            // let y = train_data.i(1..block_size + 1)?;

            // for t in 0..block_size {
            //     let context = x.i(..t + 1);
            //     let target = y.i(t);
            //     println!(
            //         "when input is {:?} the target: {:?}",
            //         context, target
            //     );
            // }
        }
    }

    Ok(())
}
