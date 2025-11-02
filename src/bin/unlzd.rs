use lzd::bit_deserializer::BitDeserializer;
use lzd::tools;

use clap::{ArgAction, Parser};
use std::fs::{File, metadata, remove_file};
use std::io::{BufReader, BufWriter, stdout};
use std::path::Path;

#[derive(Parser)]
#[command(name = "unlzd", version, author, about, long_about = None)]
struct Args {
    #[arg()]
    input_fn: String,

    #[arg(short = 'S', long = "suffix", default_value = "lzd")]
    suffix: String,

    #[arg(short, long, action = ArgAction::SetTrue)]
    stdout: bool,

    #[arg(short, long, action = ArgAction::SetTrue)]
    force: bool,

    #[arg(short, long, action = ArgAction::SetTrue)]
    remove: bool,
}

fn main() {
    let cli = Args::parse();

    let input_fn = &cli.input_fn;

    if !cli.stdout {
        let input_path = Path::new(input_fn);
        let suffix = &cli.suffix;
        if input_path.extension().unwrap().to_str() != Some(suffix) {
            eprintln!("The input extension is not {}", suffix);
            return;
        }

        let output_fn = &input_fn[..input_fn.len() - suffix.len() - 1];
        if !cli.force && metadata(output_fn).is_ok() {
            eprintln!("The output file already exists: {}", &output_fn);
            eprintln!("Please set the command option 'force' to overwrite");
            return;
        }

        let in_stream = BitDeserializer::new(BufReader::new(File::open(input_fn).unwrap()));
        let out_stream = BufWriter::new(File::create(output_fn).unwrap());
        tools::deserialize_and_decompress(in_stream, out_stream);
    } else {
        if cli.force {
            eprintln!("The option 'force' was ignored since stdout is enabled");
        }

        let out = stdout();
        let in_stream = BitDeserializer::new(BufReader::new(File::open(input_fn).unwrap()));
        let out_stream = BufWriter::new(out.lock());
        tools::deserialize_and_decompress(in_stream, out_stream);
    }

    if cli.remove {
        remove_file(input_fn).unwrap();
        eprintln!("Removed the source file {}", input_fn);
    }
}
