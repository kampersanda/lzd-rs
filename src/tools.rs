use crate::compressor;
use crate::decompressor;
use crate::deserializer::Deserializer;
use crate::misc::needed_bits;
use crate::serializer::Serializer;
use std::io::Write;

pub fn compress_and_serialize<S: Serializer>(text: &[u8], mut stream: S) -> (usize, usize) {
    let mut upper = (compressor::FACTOR_OFFSET + 1) as u64; // +1 to avoid use of factor ID zero.
    let mut nbits = needed_bits(upper);
    let mut twice = false;

    let mut written_factors = 0;

    let output = |id: usize| {
        let fid = (id + 1) as u64; // +1 to avoid use of factor ID zero.
        debug_assert!(needed_bits(fid) <= nbits);

        stream.write(fid, nbits).unwrap();
        written_factors += 1;

        if twice {
            upper += 1;
            if upper >> nbits != 0 {
                nbits += 1;
            }
        }
        twice = !twice;
    };

    let defined_factors = compressor::Compressor::run(text, output);
    stream.finish().unwrap();

    (defined_factors, written_factors)
}

pub fn deserialize_and_decompress<D, W>(mut in_stream: D, mut out_stream: W) -> usize
where
    D: Deserializer,
    W: Write,
{
    let mut upper = (compressor::FACTOR_OFFSET + 1) as u64; // +1 to avoid use of factor ID zero.
    let mut nbits = needed_bits(upper);
    let mut twice = false;

    let mut ids: Vec<usize> = Vec::new();

    loop {
        let fid = match in_stream.read(nbits) {
            Ok(v) => v,
            Err(_) => 0,
        };

        if fid == 0 {
            break;
        }

        ids.push((fid - 1) as usize);

        if twice {
            upper += 1;
            if upper >> nbits != 0 {
                nbits += 1;
            }
        }
        twice = !twice;
    }

    decompressor::Decompressor::run(&ids, |c| out_stream.write_all(&[c]).unwrap());
    out_stream.flush().unwrap();

    ids.len()
}
