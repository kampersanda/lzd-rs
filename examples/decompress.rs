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
