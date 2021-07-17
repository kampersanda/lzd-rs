use lzd::bit_deserializer::BitDeserializer;
use lzd::bit_serializer::BitSerializer;
use lzd::tools;

use clap::{App, Arg};
use std::fs::{metadata, File};
use std::io::{stdout, BufReader, BufWriter, Read, Result, Write};
use std::time;

fn main() {
    let matches = App::new("lzd")
        .version("0.1.0")
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
                .help("Overwrite the file, or not."),
        )
        .arg(
            Arg::with_name("test")
                .short("t")
                .long("test")
                .takes_value(false)
                .help("Test the compressed file, or not."),
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

    let do_test = match matches.occurrences_of("test") {
        0 => false,
        _ => true,
    };

    if !to_stdout {
        let suffix = matches.value_of("suffix").unwrap_or("lzd");
        let output_fn = format!("{}.{}", input_fn, suffix);

        if !is_force && metadata(&output_fn).is_ok() {
            eprintln!("There already exists {}.", &output_fn);
            return;
        }

        let file = File::create(&output_fn).unwrap();
        let in_stream = BitSerializer::new(BufWriter::new(file));

        let ins = time::Instant::now();
        let text = load_text(&input_fn);
        let (defined_factors, written_factors) = tools::compress_and_serialize(&text, in_stream);
        let elapsed_ms = ins.elapsed().as_millis() as f64;

        let lzd_size = metadata(&output_fn).unwrap().len();
        let cmpr_ratio_fs = lzd_size as f64 / text.len() as f64;
        let cmpr_ratio_fc = written_factors as f64 / text.len() as f64;

        eprintln!("Compression time in ms: {}", elapsed_ms);
        eprintln!("Compression time in sec: {}", elapsed_ms / 1000.0);
        eprintln!("Compression ratio in factors: {:.3}", cmpr_ratio_fc);
        eprintln!("Compression ratio in filesize: {:.3}", cmpr_ratio_fs);
        eprintln!("Number of defined LZD-factors: {}", defined_factors);
        eprintln!("Number of written LZD-factors: {}", written_factors);

        if do_test {
            eprintln!("Testing now...");
            let in_stream = BitDeserializer::new(BufReader::new(File::open(&output_fn).unwrap()));
            let mut out_stream = TextBuffer { text: Vec::new() };
            let ext_factors = tools::deserialize_and_decompress(in_stream, &mut out_stream);
            assert_eq!(written_factors, ext_factors);

            let decoded = out_stream.get_text();
            assert_eq!(text.len(), decoded.len());
            for i in 0..text.len() {
                assert_eq!(text[i], decoded[i]);
            }
            eprintln!("No problem!");
        }
    } else {
        if is_force {
            eprintln!("The option 'force' is ignored since stdout is enabled.");
        }
        if do_test {
            eprintln!("The option 'test' is ignored since stdout is enabled.");
        }

        let out = stdout();
        let stream = BitSerializer::new(BufWriter::new(out.lock()));

        let ins = time::Instant::now();
        let text = load_text(&input_fn);
        let (defined_factors, written_factors) = tools::compress_and_serialize(&text, stream);
        let elapsed_ms = ins.elapsed().as_millis() as f64;

        let cmpr_ratio_fc = written_factors as f64 / text.len() as f64;

        eprintln!("Compression time in ms: {}", elapsed_ms);
        eprintln!("Compression time in sec: {}", elapsed_ms / 1000.0);
        eprintln!("Compression ratio in factors: {:.3}", cmpr_ratio_fc);
        eprintln!("Number of defined LZD-factors: {}", defined_factors);
        eprintln!("Number of written LZD-factors: {}", written_factors);
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
