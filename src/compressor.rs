use crate::trie::{Trie, NIL_ID};

pub const FACTOR_OFFSET: usize = 256;

#[derive(Clone, Copy, Debug)]
struct Factor {
    id: usize,
    len: usize,
}

#[derive(Clone, Copy, Debug)]
struct Node {
    id: usize,
    len: usize,
}

#[derive(Debug)]
pub struct Compressor<'text> {
    trie: Trie<'text>,
    traced: Vec<Node>,
    num_factors: usize,
}

impl<'text> Compressor<'text> {
    /// Compress the input text.
    /// Retrun the number of factors.
    pub fn run<F>(text: &'text [u8], output: F) -> usize
    where
        F: FnMut(usize),
    {
        let mut worker = Compressor {
            trie: Trie::new(),
            traced: Vec::new(),
            num_factors: FACTOR_OFFSET,
        };
        worker.do_work(text, output)
    }

    /// The main routine.
    fn do_work<F>(&mut self, text: &'text [u8], mut output: F) -> usize
    where
        F: FnMut(usize),
    {
        let mut text_pos = 0;
        while text_pos < text.len() {
            let text_beg = text_pos;

            let factor1 = self.find_longest_match(&text[text_pos..], true);
            text_pos += factor1.len;

            debug_print!(factor1);
            output(factor1.id);

            if text.len() <= text_pos {
                break;
            }

            let factor2 = self.find_longest_match(&text[text_pos..], false);
            text_pos += factor2.len;

            debug_print!(factor2);
            output(factor2.id);

            if self.traced.is_empty() {
                self.insert_new_factor(0, &text[text_beg..text_pos]);
            } else {
                let mut i = 0;
                while i < self.traced.len() {
                    if text_pos < text_beg + self.traced[i].len {
                        break;
                    }
                    i += 1;
                }
                let node = self.traced[i - 1];
                self.insert_new_factor(node.id, &text[text_beg + node.len..text_pos]);
            }

            self.num_factors += 1;

            debug_print!(&text[text_beg..text_pos]);
            debug_print!(&self);
        }

        debug_print!(&self);
        self.num_factors
    }

    /// Find the deepest node traversed by the longest prefix of text.
    /// If trace == true, the traced nodes from the deepest factered node are stored.
    fn find_longest_match(&mut self, text: &'text [u8], trace: bool) -> Factor {
        assert!(!text.is_empty());

        if trace {
            self.traced.clear();
        }

        let mut node_id: usize = 0;
        let mut text_pos: usize = 0;
        let mut factor = Factor { id: NIL_ID, len: 0 };

        while text_pos < text.len() {
            node_id = self.trie.find_child(node_id, &text[text_pos..]);
            if node_id == NIL_ID {
                break;
            }
            text_pos += self.trie.get_edge_len(node_id);

            let factor_id = self.trie.get_factor_id(node_id);
            if factor_id != NIL_ID {
                factor = Factor {
                    id: factor_id,
                    len: text_pos,
                };
                if trace {
                    self.traced.clear();
                }
            }

            if trace {
                self.traced.push(Node {
                    id: node_id,
                    len: text_pos,
                });
            }
        }

        if factor.id == NIL_ID {
            factor = Factor {
                id: text[0] as usize,
                len: 1,
            }
        }
        factor
    }

    fn insert_new_factor(&mut self, node_id: usize, text: &'text [u8]) {
        if !text.is_empty() {
            self.trie.add_child(node_id, self.num_factors, text);
        } else {
            self.trie.set_factor_id(node_id, self.num_factors);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::compressor::Compressor;
    use crate::decompressor::Decompressor;
    use std::collections::HashSet;
    use std::fs::File;
    use std::io::Read;

    #[derive(PartialEq, Eq, Hash, Debug)]
    struct DoubleFactor(usize, usize);

    fn check_duplication(ids: &[usize]) {
        let mut checker: HashSet<DoubleFactor> = HashSet::new();
        for i in (1..ids.len()).step_by(2) {
            let df = DoubleFactor(ids[i - 1], ids[i]);
            assert_eq!(checker.get(&df), None);
            checker.insert(df);
        }
    }

    fn check_decompress(ids: &[usize], text: &[u8]) {
        let mut decoded = Vec::new();
        Decompressor::run(ids, |c| decoded.push(c));

        assert_eq!(text.len(), decoded.len());
        for i in 0..text.len() {
            assert_eq!(text[i], decoded[i]);
        }
    }

    #[test]
    fn tiny() {
        let text = "abaaabababaabbabab".as_bytes();

        let mut ids: Vec<usize> = Vec::new();
        Compressor::run(text, |id: usize| {
            ids.push(id);
        });

        check_duplication(&ids);
        check_decompress(&ids, text);
    }

    #[test]
    fn short() {
        let text = "ddadabaddaccdababddccccaccdcaddccacaaabcdbbddacbcbbbdcaadbbcbaddcbcdbaacbbddddbcdbdabdbcbbbcdcdcbada".as_bytes();

        let mut ids: Vec<usize> = Vec::new();
        Compressor::run(text, |id: usize| {
            ids.push(id);
        });

        check_duplication(&ids);
        check_decompress(&ids, text);
    }

    #[test]
    fn alice() {
        let mut file = File::open("data/alice29.txt").expect("File not found");
        let mut text: Vec<u8> = Vec::new();
        let _ = file.read_to_end(&mut text).unwrap();

        let mut ids: Vec<usize> = Vec::new();
        Compressor::run(&text, |id: usize| {
            ids.push(id);
        });

        check_duplication(&ids);
        check_decompress(&ids, &text);
    }
}
