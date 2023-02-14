use std::ops::Index;

type BackingType = u32;
const BITS: usize = BackingType::BITS as usize;

pub struct ExpandableBitVec {
    backing: Vec<BackingType>,
}

impl ExpandableBitVec {
    pub fn new(size: usize) -> Self {
        Self {
            backing: vec![0; size / BITS + 1],
        }
    }

    pub fn extend_to_size(&mut self, size: usize) {
        let old_len = self.backing.len();
        let new_len = size / BITS + 1;
        if new_len > old_len {
            self.backing.append(&mut vec![0; new_len - old_len]);
        }
    }

    pub fn index(&self, index: usize) -> bool {
        let vec_ind = index / BITS;
        let Some(el) = self.backing.get(vec_ind) else {
            panic!("Indexing Out of Bounds!");
        };

        let backing_ind = index % BITS;

        el & (1 << backing_ind) > 0
    }

    pub fn set(&mut self, index: usize) {
        self.extend_to_size(index + 1);

        let vec_ind = index / BITS;
        let backing_ind = index % BITS;

        self.backing[vec_ind] |= 1 << backing_ind;
    }

    pub fn unset(&mut self, index: usize) {
        self.extend_to_size(index + 1);

        let vec_ind = index / BITS;
        let backing_ind = index % BITS;

        self.backing[vec_ind] &= BackingType::MAX - (1 << backing_ind);
    }

    pub fn clear(&mut self) {
        for el in &mut self.backing {
            *el = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bitvec::ExpandableBitVec;

    #[test]
    pub fn test_bitvec() {
        let mut bv = ExpandableBitVec::new(10);
        for i in 0..10 {
            eprintln!("Checking default {i}");
            assert!(!bv.index(i));
        }

        for i in (0..30).filter(|i| i % 3 == 0) {
            eprintln!("Set %3 {i}");
            bv.set(i);
        }
        for i in 0..30 {
            eprintln!("Check %3 {i}");
            if i % 3 == 0 {
                assert!(bv.index(i));
            } else {
                assert!(!bv.index(i));
            }
        }

        for i in (0..30).filter(|i| i % 6 == 0) {
            eprintln!("Unset %6 {i}");
            bv.unset(i);
        }
        for i in 0..30 {
            eprintln!("Check %6 {i}");
            if i % 6 == 0 {
                assert!(!bv.index(i));
            } else if i % 3 == 0 {
                assert!(bv.index(i));
            } else {
                assert!(!bv.index(i));
            }
        }
    }
}
