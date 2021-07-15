#[macro_use]
mod macros;

mod basic;
mod compressor;
mod trie;

use crate::compressor::Compressor;
use clap::{App, Arg};
use std::fs::File;
use std::io::Read;

fn main() {
    let matches = App::new("lzd")
        .version("0.1.0")
        .author("Kampersanda <shnsk.knd@gmail.com>")
        .arg(
            Arg::with_name("input")
                .help("input file name to be compressed")
                .required(true),
        )
        .get_matches();

    let input = matches.value_of("input").unwrap();

    let mut file = File::open(input).expect("File not found");
    let mut text: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut text).unwrap();

    Compressor::run(&text, encode);
}

fn encode(factor_id: usize, factor_count: usize) {
    println!("{},{}", factor_id, factor_count);
}
