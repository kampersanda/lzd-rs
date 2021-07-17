use std::io::Result;

pub trait Serializer {
    fn write(&mut self, x: u64, nbits: usize) -> Result<()>;
    fn finish(&mut self) -> Result<()>;
}
