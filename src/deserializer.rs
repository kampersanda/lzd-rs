use crate::misc::bytes_for;

use std::fs::File;
use std::io::{BufReader, Read, Result};

///! Bit-wise deserializer.
pub struct Deserializer {
    stream: BufReader<File>,
    buffer: [u8; 1],
    cursor: usize,
}

impl Deserializer {
    pub fn new(filepath: &str) -> Result<Deserializer> {
        let file = File::open(filepath)?;
        let stream = BufReader::new(file);
        Ok(Deserializer {
            stream: stream,
            buffer: [0; 1],
            cursor: 0,
        })
    }

    pub fn read(&mut self, mut nbits: usize) -> Result<u64> {
        assert!(self.cursor < 8);
        assert!(nbits != 0 && nbits <= 64);

        let mut i: usize = 0;
        let mut x: u64 = 0;

        if self.cursor != 0 {
            x = (self.buffer[0] >> self.cursor) as u64;
            if self.cursor + nbits <= 8 {
                x &= (1 << nbits) - 1;
                self.cursor = (self.cursor + nbits) % 8;
                return Ok(x);
            }
            i = 8 - self.cursor;
            nbits -= i;
        }

        let read_nbytes = bytes_for(nbits);
        assert_ne!(read_nbytes, 0);

        for _ in 1..read_nbytes {
            self.stream.read_exact(&mut self.buffer)?;
            x |= (self.buffer[0] as u64) << i;
            i += 8;
            nbits -= 8;
        }
        assert!(nbits <= 8);

        self.stream.read_exact(&mut self.buffer)?;
        let byte = self.buffer[0] as u64;
        let mask = (1 << nbits) - 1;

        x |= (byte & mask) << i;
        self.cursor = nbits % 8;

        Ok(x)
    }
}

#[cfg(test)]
mod tests {
    use crate::deserializer::Deserializer;
    use crate::misc::needed_bits;

    use std::fs::{remove_file, File};
    use std::io::Write;

    #[test]
    fn tiny() {
        let tmpfile = "deserialized.bin";

        let ints = [7, 45, 34, 255, 256, 3, 500000, 444];
        let bytes = [111, 197, 127, 128, 131, 132, 158, 55]; // serialized ints
        {
            let mut file = File::create(tmpfile).unwrap();
            file.write_all(&bytes).unwrap();
        }

        {
            let mut des = Deserializer::new(tmpfile).unwrap();
            for x in ints {
                let nbits = needed_bits(x);
                let y = des.read(nbits).unwrap();
                assert_eq!(x, y);
            }
        }

        remove_file(tmpfile).unwrap();
    }
}
