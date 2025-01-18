// use nanogpt_rs::utils::{
//     create_mappings, encode, get_unique_characters, read_file_utf8,
// };

use candle_core::{Device::Cpu, Tensor};
// use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // try to set seed first
    // Cpu.set_seed(13);
    Cpu.set_seed(12)?;

    let a = Tensor::rand(0f32, 10f32, 20, &Cpu)?;
    println!("{:?}", a.to_vec1::<f32>());

    // let file_path = Path::new("./data/input.txt"); // Replace with your file path

    // match read_file_utf8(file_path) {
    //     Err(error) => {
    //         eprintln!("Error reading file: {}", error);
    //     }

    //     Ok(contents) => {
    //         let chars = get_unique_characters(&contents);

    //         let (stoi, _) = create_mappings(&chars);

    //         // try to use candle_core
    //         let encoded: Vec<f32> =
    //             encode(&contents, &stoi).iter().map(|&x| x as f32).collect();

    //         let data = Tensor::new(encoded, &Cpu)?;
    //         let n =
    //             (0.9 * data.shape().dim(0).unwrap() as f32).round() as usize;
    //         println!("{:?}", n);

    //         let train_data = data.i(..n)?;
    //         let valid_data = data.i(n..)?;
    //     }
    // }

    Ok(())
}
