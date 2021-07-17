# lzd-rs
LZ double-factor factorization in Rust

## Examples

```rust
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
```

The output will be

```
factors: [97, 98, 97, 97, 256, 256, 256, 257, 98, 98, 258]
defined_factors: 261
```

