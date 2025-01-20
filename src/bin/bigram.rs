use candle_nn::Embedding;

// Define a struct named Example
struct BigramLanguageModel {
    token_embedding_table: Embedding,
}

impl BigramLanguageModel {
    // fn new(vocab_size: i32) -> Self {
    //     Self {
    //         token_embedding_table: Embedding(vocab_size, vocab_size),
    //     }
    // }

    fn forward(self, idx: i32, target: i32) -> i32 {
        0
    }
}

fn main() {}
