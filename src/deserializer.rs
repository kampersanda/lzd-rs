use std::io::Result;

pub trait Deserializer {
    fn read(&mut self, nbits: usize) -> Result<u64>;
}
