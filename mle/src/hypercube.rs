use bitvec::{field::BitField, vec::BitVec};

/// Represents a complete Boolean hypercube.
/// Stores v-bit tuples in reverse order.
pub struct Hypercube {
    tuples: Vec<BitVec>,
}

impl Hypercube {
    /// Constructs a new [`Hypercube`] of size 2^v.
    pub fn new(v: u32) -> Self {
        let sz = 2u32.pow(v) as usize;
        let tuples = (0..sz)
            .map(|i| {
                let mut tuple = BitVec::with_capacity(v as usize);
                tuple.resize(v as usize, false);
                tuple[..].store(i);
                tuple
            })
            .rev()
            .collect::<Vec<_>>();

        Self { tuples }
    }
}

impl Iterator for Hypercube {
    type Item = BitVec;

    fn next(&mut self) -> Option<Self::Item> {
        self.tuples.pop()
    }
}
