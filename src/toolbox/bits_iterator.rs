struct BitsIterator {
    num: usize,
    mask: usize,
}

impl BitsIterator {
    fn new_with_length(num: usize, length: u32) -> Self {
        BitsIterator {
            num,
            mask: 1 << (length - 1),
        }
    }
}

impl Iterator for BitsIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.mask {
            0 => None,
            _ => {
                let r = self.num & self.mask;
                self.mask >>= 1;
                Some(if r == 0 { 0 } else { 1 })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_iterator_using_new_with_length() {
        let iterator = BitsIterator::new_with_length(5, 8);

        assert_eq!(iterator.collect::<Vec<_>>(), vec![0, 0, 0, 0, 0, 1, 0, 1]);
    }
}