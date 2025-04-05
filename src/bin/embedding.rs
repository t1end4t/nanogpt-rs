use candle_core::{DType, Device, Result};
use candle_nn::{Embedding, VarBuilder, VarMap};

use rand::prelude::ThreadRng;

pub struct SimpleBigramLanguageModel {
    /// Even though Karpathy calls the final bigram model simple, this is the intermediate version in the notebook.
    vocab_size: usize,
    token_embedding_table: Embedding,
    var_map: VarMap,
    rng: ThreadRng,
}

impl SimpleBigramLanguageModel {
    pub fn new(vocab_size: usize, hidden_size: usize, device: &Device) -> Self {
        // create embedding table even lack of variables
        let var_map = VarMap::new();
        let var_builder = VarBuilder::from_varmap(&var_map, DType::F32, device);
        let embeddings = var_builder
            .get((vocab_size, hidden_size), "embeddings")
            .unwrap();
        let token_embedding_table = Embedding::new(embeddings, hidden_size);
        let rng = rand::rng();
        Self {
            vocab_size,
            token_embedding_table,
            var_map,
            rng,
        }
    }
}

fn main() -> Result<()> {
    // now we will use translate first
    let device = Device::Cpu;
    let embedding = SimpleBigramLanguageModel::new(10, 3, &device);

    Ok(())
}
