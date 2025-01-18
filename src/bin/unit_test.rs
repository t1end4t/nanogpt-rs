use candle_core::Device::Cpu;
use candle_core::Tensor;

fn main() {
    let data = Tensor::new(10f32, &Cpu);
}
