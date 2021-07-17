use lzd::compressor::{Compressor, FACTOR_OFFSET};
use lzd::misc::needed_bits;
use lzd::serializer::Serializer;

use clap::{App, Arg};
use std::fs::File;
use std::io::Read;

fn main() {
    let matches = App::new("lzd")
        .version("0.1.0")
        .author("Kampersanda <shnsk.knd@gmail.com>")
        .arg(
            Arg::with_name("input_fn")
                .help("input file name to be compressed")
                .required(true),
        )
        .arg(
            Arg::with_name("output_fn")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("output file name of compressed file"),
        )
        .get_matches();

    let input_fn = matches.value_of("input_fn").unwrap();

    let default_output_fn = format!("{}.lzd", input_fn);
    let output_fn = matches.value_of("output_fn").unwrap_or(&default_output_fn);

    let mut text: Vec<u8> = Vec::new();
    {
        let mut file = File::open(input_fn).unwrap();
        let _ = file.read_to_end(&mut text).unwrap();
    }

    let mut ser = Serializer::new(&output_fn).unwrap();
    let mut upper = (FACTOR_OFFSET + 1) as u64; // +1 to avoid use of factor ID zero.
    let mut nbits = needed_bits(upper);
    let mut twice = false;

    let outputter = |id: usize| {
        let fid = (id + 1) as u64; // +1 to avoid use of factor ID zero.
        assert!(needed_bits(fid) <= nbits);

        ser.write(fid, nbits).unwrap();
        if twice {
            upper += 1;
            nbits = needed_bits(upper);
        }
        twice = !twice;
    };

    Compressor::run(&text, outputter);
    ser.finish().unwrap();
}
