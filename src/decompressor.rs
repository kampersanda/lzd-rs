use crate::compressor;

pub const FACTOR_OFFSET: usize = compressor::FACTOR_OFFSET;

pub struct Decompressor<'ids> {
    ids: &'ids [usize],
}

/// LZD decompressor.
impl<'ids> Decompressor<'ids> {
    pub fn run<F>(ids: &'ids [usize], output: F)
    where
        F: FnMut(u8),
    {
        let mut worker = Decompressor { ids: ids };
        worker.do_work(output);
    }

    /// The main routine.
    fn do_work<F>(&mut self, mut output: F)
    where
        F: FnMut(u8),
    {
        for id in self.ids {
            output = self.decode(*id, output);
        }
    }

    fn decode<F>(&mut self, mut id: usize, mut output: F) -> F
    where
        F: FnMut(u8),
    {
        if id < FACTOR_OFFSET {
            output(id as u8);
        } else {
            id -= FACTOR_OFFSET;
            debug_assert!(id * 2 + 1 < self.ids.len());
            output = self.decode(self.ids[id * 2], output);
            output = self.decode(self.ids[id * 2 + 1], output);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use crate::decompressor::{Decompressor, FACTOR_OFFSET};

    #[test]
    fn tiny() {
        let text = "abaaabababaabbabab".as_bytes();
        let ids = vec![
            'a' as usize,
            'b' as usize,
            'a' as usize,
            'a' as usize,
            FACTOR_OFFSET,     // ab
            FACTOR_OFFSET,     // ab
            FACTOR_OFFSET,     // ab
            FACTOR_OFFSET + 1, // aa
            'b' as usize,
            'b' as usize,
            FACTOR_OFFSET + 2, // abab
        ];

        let mut decoded = Vec::new();
        Decompressor::run(&ids, |c| decoded.push(c));

        assert_eq!(text.len(), decoded.len());
        for i in 0..text.len() {
            assert_eq!(text[i], decoded[i]);
        }
    }
}
