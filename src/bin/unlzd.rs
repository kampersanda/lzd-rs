use lzd::bit_deserializer::BitDeserializer;
use lzd::tools;

use clap::{App, Arg};
use std::fs::{metadata, remove_file, File};
use std::io::{stdout, BufReader, BufWriter};
use std::path::Path;

fn main() {
    let matches = App::new("unlzd")
        .version("0.1.1")
        .author("Kampersanda <shnsk.knd@gmail.com>")
        .arg(
            Arg::with_name("input_fn")
                .help("input file name to be uncompressed.")
                .required(true),
        )
        .arg(
            Arg::with_name("suffix")
                .short("S")
                .long("suffix")
                .takes_value(true)
                .help("Extension of input file name (=lzd)."),
        )
        .arg(
            Arg::with_name("stdout")
                .short("c")
                .long("stdout")
                .takes_value(false)
                .help("Write the result into the stdout, or not."),
        )
        .arg(
            Arg::with_name("force")
                .short("f")
                .long("force")
                .takes_value(false)
                .help("Forcibly overwrite the file, or not."),
        )
        .arg(
            Arg::with_name("remove")
                .short("r")
                .long("remove")
                .takes_value(false)
                .help("Remove the source file after decompression, or not."),
        )
        .get_matches();

    let input_fn = matches.value_of("input_fn").unwrap();

    let to_stdout = match matches.occurrences_of("stdout") {
        0 => false,
        _ => true,
    };

    let is_force = match matches.occurrences_of("force") {
        0 => false,
        _ => true,
    };

    let do_remove = match matches.occurrences_of("remove") {
        0 => false,
        _ => true,
    };

    if !to_stdout {
        let input_path = Path::new(input_fn);
        let suffix = matches.value_of("suffix").unwrap_or("lzd");
        if input_path.extension().unwrap() != suffix {
            eprintln!("The input extension is not {}.", suffix);
            return;
        }

        let output_fn = &input_fn[..input_fn.len() - suffix.len() - 1];
        if !is_force && metadata(&output_fn).is_ok() {
            eprintln!("There already exists {}.", &output_fn);
            return;
        }

        let in_stream = BitDeserializer::new(BufReader::new(File::open(&input_fn).unwrap()));
        let out_stream = BufWriter::new(File::create(&output_fn).unwrap());
        tools::deserialize_and_decompress(in_stream, out_stream);
    } else {
        if is_force {
            eprintln!("The option 'force' is ignored since stdout is enabled.");
        }

        let out = stdout();
        let in_stream = BitDeserializer::new(BufReader::new(File::open(&input_fn).unwrap()));
        let out_stream = BufWriter::new(out.lock());
        tools::deserialize_and_decompress(in_stream, out_stream);
    }

    if do_remove {
        remove_file(input_fn).unwrap();
        eprintln!("Removed the source file {}", input_fn);
    }
}
