use lzd::compressor::Compressor;

fn main() {
    // Input text
    let text = "abaaabababaabbabab".as_bytes();

    // Factorization
    let mut factors = Vec::new();
    let defined_factors = Compressor::run(text, |id: usize| {
        factors.push(id);
    });

    // Output factors
    println!("factors: {:?}", factors);

    // Statistics
    println!("defined_factors: {:?}", defined_factors);
}
