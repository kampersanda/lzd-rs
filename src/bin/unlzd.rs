use lzd::decompressor::{Decompressor, FACTOR_OFFSET};
use lzd::deserializer::Deserializer;
use lzd::misc::needed_bits;

use clap::{App, Arg};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let matches = App::new("unlzd")
        .version("0.1.0")
        .author("Kampersanda <shnsk.knd@gmail.com>")
        .arg(
            Arg::with_name("input_fn")
                .help("input file name to be decompressed (whose ext is .lzd)")
                .required(true),
        )
        .arg(
            Arg::with_name("output_fn")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("output file name of uncompressed file"),
        )
        .get_matches();

    let input_fn = matches.value_of("input_fn").unwrap();

    let input_path = Path::new(input_fn);
    if input_path.extension().unwrap() != "lzd" {
        panic!("The input extension is not '.lzd'.");
    }

    let default_output_fn = input_path.file_stem().unwrap().to_str().unwrap();
    let output_fn = matches.value_of("output_fn").unwrap_or(&default_output_fn);

    let mut ids: Vec<usize> = Vec::new();
    {
        let mut deser = Deserializer::new(input_fn).unwrap();
        let mut upper = (FACTOR_OFFSET + 1) as u64; // +1 to avoid use of factor ID zero.
        let mut nbits = needed_bits(upper);
        let mut twice = false;

        loop {
            let fid = match deser.read(nbits) {
                Ok(v) => v,
                Err(_) => 0,
            };

            if fid == 0 {
                break;
            }

            ids.push((fid - 1) as usize);

            if twice {
                upper += 1;
                nbits = needed_bits(upper);
            }
            twice = !twice;
        }
    }

    let mut stream = BufWriter::new(File::create(output_fn).unwrap());
    Decompressor::run(&ids, |c| stream.write_all(&[c]).unwrap());
}
