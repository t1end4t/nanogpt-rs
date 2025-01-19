use nanogpt_rs::{
    get_row_values_in_range, get_value, pretty_print_2d_vector,
    utils::{
        create_mappings, encode, get_batch, get_unique_characters,
        read_file_utf8,
    },
};

use candle_core::{Device::Cpu, IndexOp, Tensor};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: try to set seed first

    let a = Tensor::rand(0f32, 10f32, 20, &Cpu)?;
    // println!("{:?}", a.to_vec1::<f32>());

    let batch_size = 4;
    let block_size = 8;

    let file_path = Path::new("./data/input.txt"); // Replace with your file path

    match read_file_utf8(file_path) {
        Err(error) => {
            eprintln!("Error reading file: {}", error);
        }

        Ok(contents) => {
            let chars = get_unique_characters(&contents);

            let (stoi, _) = create_mappings(&chars);

            // try to use candle_core
            let encoded: Vec<f32> =
                encode(&contents, &stoi).iter().map(|&x| x as f32).collect();

            let data = Tensor::new(encoded, &Cpu)?;

            //NOTE: different from python
            let n = ((0.9 * data.shape().dim(0).unwrap() as f32).round() - 1f32)
                as usize;
            // println!("{:?}", n);

            let train_data = data.i(..n)?;
            let valid_data = data.i(n..)?;

            let (xb, yb) = get_batch(
                "train", block_size, batch_size, train_data, valid_data,
            );

            println!("inputs:");
            println!("{:?}", xb.shape());
            pretty_print_2d_vector(&xb.to_vec2::<f32>().unwrap());

            println!("targets:");
            println!("{:?}", yb.shape());
            pretty_print_2d_vector(&yb.to_vec2::<f32>().unwrap());

            for b in 0..batch_size {
                for t in 0..block_size {
                    let context = get_row_values_in_range(
                        &xb.to_vec2::<f32>().unwrap(),
                        b,
                        0..(t + 1),
                    );

                    let target = get_value(&yb.to_vec2::<f32>().unwrap(), b, t);

                    println!(
                        "when input is {:?} the target: {:?}",
                        context, target
                    );
                }
            }
        }
    }
    Ok(())
}
