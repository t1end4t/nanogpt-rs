use candle_core::{Device, Result, Tensor};

fn main() -> Result<()> {
    let device = Device::Cpu;
    //NOTE: failed
    // let _ = device.set_seed(64)?;

    let a = Tensor::rand(0f32, 10f32, 20, &device)?;
    println!("{:?}", a.to_vec1::<f32>());

    Ok(())
}
