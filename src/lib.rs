//! # lzd-rs
//!
//! This library provides a Rust implementation of LZ double-factor factorization, an efficient grammar-based compression algorithm, proposed in the paper:
//!
//! > K Goto, H Bannai, S Inenaga, and M Takeda. **LZD Factorization: Simple and Practical Online Grammar Compression with Variable-to-Fixed Encoding.** In *CPM*, 2015.
//!
//! ## Examples
//!
//! ### Factorization
//!
//! ```rust
//! use lzd::compressor::Compressor;
//!
//! fn main() {
//!     // Input text
//!     let text = "abaaabababaabbabab".as_bytes();
//!
//!     // Factorization
//!     let mut factors = Vec::new();
//!     let defined_factors = Compressor::run(text, |id: usize| {
//!         factors.push(id);
//!     });
//!
//!     // Output factors
//!     println!("factors: {:?}", factors);
//!
//!     // Statistics
//!     println!("defined_factors: {:?}", defined_factors);
//! }
//! ```
//!
//! The output will be
//!
//! ```
//! factors: [97, 98, 97, 97, 256, 256, 256, 257, 98, 98, 258]
//! defined_factors: 261
//! ```
//!
//! *NOTE:* In this implementation, all 256 single characters are predefined as factors, so the number of factors defined will become 261.
//!
//! ### Defactorization
//!
//! ```rust
//! use lzd::decompressor::Decompressor;
//!
//! fn main() {
//!     // Input text
//!     let factors = [97, 98, 97, 97, 256, 256, 256, 257, 98, 98, 258];
//!
//!     // Defactorization
//!     let mut text = String::new();
//!     Decompressor::run(&factors, |c: u8| {
//!         text.push(c as char);
//!     });
//!
//!     // Decoded text
//!     println!("text: {:?}", text);
//! }
//! ```
//!
//! The output will be
//!
//! ```
//! text: "abaaabababaabbabab"
//! ```
//!
#[macro_use]
mod macros;

pub mod bit_deserializer;
pub mod bit_serializer;
pub mod compressor;
pub mod decompressor;
pub mod deserializer;
pub mod misc;
pub mod serializer;
pub mod tools;
pub mod trie;
