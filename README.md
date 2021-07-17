# lzd-rs
This library provides a Rust implementation of LZ double-factor factorization, an efficient grammar-based compression algorithm, proposed in the paper:

> K Goto, H Bannai, S Inenaga, and M Takeda. **LZD Factorization: Simple and Practical Online Grammar Compression with Variable-to-Fixed Encoding.** In *CPM*, 2015.

## Examples

### Factorization

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

*NOTE:* In this implementation, all 256 single characters are predefined as factors, so the number of factors defined will become 261.

### Defactorization

```rust
use lzd::decompressor::Decompressor;

fn main() {
    // Input text
    let factors = [97, 98, 97, 97, 256, 256, 256, 257, 98, 98, 258];

    // Defactorization
    let mut text = String::new();
    Decompressor::run(&factors, |c: u8| {
        text.push(c as char);
    });

    // Decoded text
    println!("text: {:?}", text);
}
```

The output will be

```
text: "abaaabababaabbabab"
```

## Commnad line tools

This library provides two command line tools for compression and decompression. The tools will print the command line options by specifying the parameter `-h`. In the tools, LZ factors are serialized into a binary stream, in the same manner as `tdc::BitCorder` of [tudocomp](https://tudocomp.github.io/).

### `lzd` command

It compresses an input data and writes the result into a file with the extension `lzd`. In the following case, `english.50MB.lzd` will be written as the compressed file.

```sh
$ ./target/release/lzd english.50MB
Compression time in ms: 12216
Compression time in sec: 12.216
Compression ratio in factors: 0.121
Compression ratio in filesize: 0.313
Number of defined LZD-factors: 3177320
Number of written LZD-factors: 6354129
```

### `unlzd` command

It decompresses a compressed file and writes the original data into a file without the extension `lzd`. In the following case, `english.50MB` will be written as the decompressed file.

```sh
$ ./target/release/unlzd english.50MB.lzd
Decompression time in ms: 1573
Decompression time in sec: 1.573
Number of extracted LZD-factors: 6354129
```

## Licensing

This library is free software provided under MIT.
