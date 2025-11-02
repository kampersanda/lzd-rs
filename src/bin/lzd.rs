use lzd::bit_deserializer::BitDeserializer;
use lzd::bit_serializer::BitSerializer;
use lzd::tools;

use clap::{App, Arg};
use std::fs::{metadata, remove_file, File};
use std::io::{stdout, BufReader, BufWriter, Read, Result, Write};

fn main() {
    let matches = App::new("lzd")
        .version("0.1.1")
        .author("Kampersanda <shnsk.knd@gmail.com>")
        .arg(
            Arg::with_name("input_fn")
                .help("Input file name to be compressed.")
                .required(true),
        )
        .arg(
            Arg::with_name("suffix")
                .short("S")
                .long("suffix")
                .takes_value(true)
                .help("Extension of output file name (=lzd)."),
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
            Arg::with_name("test")
                .short("t")
                .long("test")
                .takes_value(false)
                .help("Test the compressed file, or not."),
        )
        .arg(
            Arg::with_name("remove")
                .short("r")
                .long("remove")
                .takes_value(false)
                .help("Remove the source file after compression, or not."),
        )
        .get_matches();

    let input_fn = matches.value_of("input_fn").unwrap();

    let to_stdout = !matches!(matches.occurrences_of("stdout"), 0);

    let is_force = !matches!(matches.occurrences_of("force"), 0);

    let do_test = !matches!(matches.occurrences_of("test"), 0);

    let do_remove = !matches!(matches.occurrences_of("remove"), 0);

    if !to_stdout {
        let suffix = matches.value_of("suffix").unwrap_or("lzd");
        let output_fn = format!("{}.{}", input_fn, suffix);
        eprintln!("Compressed filename will be {}", &output_fn);

        if !is_force && metadata(&output_fn).is_ok() {
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

        if do_test {
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
        if is_force {
            eprintln!("The option 'force' was ignored since stdout is enabled");
        }
        if do_test {
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

    if do_remove {
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
