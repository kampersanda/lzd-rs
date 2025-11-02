use lzd::bit_deserializer::BitDeserializer;
use lzd::bit_serializer::BitSerializer;
use lzd::tools;

use clap::{ArgAction, Parser};
use std::fs::{File, metadata, remove_file};
use std::io::{BufReader, BufWriter, Read, Result, Write, stdout};

#[derive(Parser)]
#[command(name = "lzd", version, author, about, long_about = None)]
struct Args {
    #[arg(long)]
    input_fn: String,

    #[arg(short = 'S', long = "suffix", default_value = "lzd")]
    suffix: String,

    #[arg(short, long, action = ArgAction::SetTrue)]
    stdout: bool,

    #[arg(short, long, action = ArgAction::SetTrue)]
    force: bool,

    #[arg(short, long, action = ArgAction::SetTrue)]
    test: bool,

    #[arg(short, long, action = ArgAction::SetTrue)]
    remove: bool,
}

fn main() {
    let cli = Args::parse();

    let input_fn = &cli.input_fn;

    if !cli.stdout {
        let suffix = &cli.suffix;
        let output_fn = format!("{}.{}", input_fn, suffix);
        eprintln!("Compressed filename will be {}", &output_fn);

        if cli.force && metadata(&output_fn).is_ok() {
            eprintln!("The output file already exists: {}", &output_fn);
            eprintln!("Please set the command option 'force' to overwrite");
            return;
        }

        let file = File::create(&output_fn).unwrap();
        let in_stream = BitSerializer::new(BufWriter::new(file));

        let text = load_text(input_fn);
        let (defined_factors, written_factors) = tools::compress_and_serialize(&text, in_stream);

        let compressed_size = metadata(&output_fn).unwrap().len();
        let cmpr_ratio_fs = compressed_size as f64 / text.len() as f64;
        let cmpr_ratio_fc = written_factors as f64 / text.len() as f64;

        eprintln!(
            "{} bytes were compressed into {} bytes ({:.2}%)",
            text.len(),
            compressed_size,
            cmpr_ratio_fs * 100.0
        );
        eprintln!(
            "{} characters were factorized into {} LZD-factors ({:.2}%)",
            text.len(),
            written_factors,
            cmpr_ratio_fc * 100.0
        );
        eprintln!("{} LZD-factors were defined", defined_factors);

        if cli.test {
            let in_stream = BitDeserializer::new(BufReader::new(File::open(&output_fn).unwrap()));
            let mut out_stream = TextBuffer { text: Vec::new() };
            let ext_factors = tools::deserialize_and_decompress(in_stream, &mut out_stream);
            assert_eq!(written_factors, ext_factors);

            let decoded = out_stream.get_text();
            assert_eq!(text.len(), decoded.len());
            for i in 0..text.len() {
                assert_eq!(text[i], decoded[i]);
            }
            eprintln!("Passed the decompression test!");
        }
    } else {
        if cli.force {
            eprintln!("The option 'force' was ignored since stdout is enabled");
        }
        if cli.test {
            eprintln!("The option 'test' was ignored since stdout is enabled");
        }

        let out = stdout();
        let stream = BitSerializer::new(BufWriter::new(out.lock()));

        let text = load_text(input_fn);
        let (defined_factors, written_factors) = tools::compress_and_serialize(&text, stream);

        let cmpr_ratio_fc = written_factors as f64 / text.len() as f64;

        eprintln!(
            "{} characters were factorized into {} LZD-factors ({:.2}%)",
            text.len(),
            written_factors,
            cmpr_ratio_fc * 100.0
        );
        eprintln!("{} LZD-factors were defined", defined_factors);
    }

    if cli.remove {
        remove_file(input_fn).unwrap();
        eprintln!("Removed the source file {}", input_fn);
    }
}

fn load_text(input_fn: &str) -> Vec<u8> {
    let mut text: Vec<u8> = Vec::new();
    let mut file = File::open(input_fn).unwrap();
    let _ = file.read_to_end(&mut text).unwrap();
    text
}

struct TextBuffer {
    text: Vec<u8>,
}

impl TextBuffer {
    pub fn get_text(&self) -> &[u8] {
        &self.text
    }
}

impl Write for TextBuffer {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.text.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }

    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        self.text.extend_from_slice(buf);
        Ok(())
    }
}
