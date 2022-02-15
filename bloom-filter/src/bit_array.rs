#[derive(Debug, PartialEq)]
pub struct BitArray {
    pub inner: Vec<bool>,
}

impl BitArray {

    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        Self {
            inner: BitArray::fill(size),
        }
    }

    #[allow(dead_code)]
    fn fill(size: usize) -> Vec<bool> {
        let mut v = Vec::with_capacity(size);
        v.resize(size, false);
        v
    }

    #[allow(dead_code)]
    pub fn flip(&mut self, _location: usize) -> bool {
        assert!(_location <= self.inner.len(), "index not found");
        self.inner[_location] = true;
        true
    }

    #[allow(dead_code)]
    pub fn get(&self, _location: usize) -> bool {
        assert!(_location <= self.inner.len(), "index not found");
        self.inner[_location]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_create_array_of_size() {
        assert_eq!(
            BitArray::new(126),
            BitArray {
                inner: BitArray::fill(126)
            }
        );
    }

    #[test]
    fn it_should_set_1() {
        let mut b = BitArray::new(126);
        assert!(b.flip(12));
    }

    #[test]
    fn it_should_get_value() {
        let mut b = BitArray::new(126);
        b.flip(12);
        assert!(b.get(12));
    }
}
