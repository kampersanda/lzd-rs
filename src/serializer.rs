use std::fs::File;
use std::io::{BufWriter, Result, Write};

///! Bit-wise serializer.
pub struct Serializer {
    stream: BufWriter<File>,
    buffer: [u8; 8],
    cursor: usize,
}

impl Serializer {
    pub fn new(filepath: &str) -> Result<Serializer> {
        let file = File::create(filepath)?;
        let stream = BufWriter::new(file);
        Ok(Serializer {
            stream: stream,
            buffer: [0; 8],
            cursor: 0,
        })
    }

    pub fn write(&mut self, mut x: u64, mut nbits: usize) -> Result<()> {
        assert!(self.cursor < 8);
        assert!(nbits != 0 && nbits <= 64);

        if self.cursor + nbits < 8 {
            let mask = (1 << nbits) - 1;
            self.buffer[0] |= ((x & mask) << self.cursor) as u8;
            self.cursor += nbits;
            return Ok(());
        }

        let mut i = 0;

        if self.cursor != 0 {
            let rest = 8 - self.cursor;
            let mask = (1 << rest) - 1;
            self.buffer[i] |= ((x & mask) << self.cursor) as u8;
            x >>= rest;
            nbits -= rest;
            i += 1;
        }

        while nbits >= 8 {
            self.buffer[i] = (x & 0xFF) as u8;
            x >>= 8;
            nbits -= 8;
            i += 1;
        }

        self.stream.write_all(&self.buffer[..i])?;

        if nbits == 0 {
            self.buffer[0] = 0;
            self.cursor = 0;
        } else {
            let mask = (1 << nbits) - 1;
            self.buffer[0] = (x & mask) as u8;
            self.cursor = nbits;
        }

        Ok(())
    }

    pub fn finish(&mut self) -> Result<()> {
        self.stream.write_all(&self.buffer[..1])?;
        self.stream.flush().unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::misc::needed_bits;
    use crate::serializer::Serializer;

    use std::fs::{remove_file, File};
    use std::io::Read;

    #[test]
    fn tiny() {
        let tmpfile = "serialized.bin";

        let ints = [7, 45, 34, 255, 256, 3, 500000, 444];
        {
            let mut ser = Serializer::new(tmpfile).unwrap();
            for x in &ints {
                let nbits = needed_bits(*x);
                ser.write(*x, nbits).unwrap();
            }
            ser.finish().unwrap();
        }

        let mut serialized: u64 = 0;
        {
            let mut buffer = [0; 8];
            let mut file = File::open(tmpfile).unwrap();
            file.read_exact(&mut buffer).unwrap();
            for i in 0..buffer.len() {
                serialized |= (buffer[i] as u64) << (i * 8);
            }
        }

        for x in &ints {
            let nbits = needed_bits(*x);
            let mask = (1 << nbits) - 1;
            let y = serialized & mask;
            assert_eq!(*x, y);
            serialized >>= nbits;
        }

        remove_file(tmpfile).unwrap();
    }
}
