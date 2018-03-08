pub trait Bits: Clone {
    fn max_capacity() -> i32;
    fn allocate(size: i32) -> Self;
    fn disjoint(&self, other: &Self) -> bool;
    fn update(&mut self, other: &Self);
    fn set(&mut self, idx: i32);
    fn unset(&mut self, idx: i32);
    fn lowest_unset_bit(&self) -> i32;
    fn lowest_set_bit(&self) -> i32;
    fn is_empty(&self) -> bool;
}

impl Bits for u64 {
    fn max_capacity() -> i32 {
        64
    }
    fn allocate(size: i32) -> u64 {
        assert!(size <= 64);
        0u64
    }
    fn disjoint(&self, other: &u64) -> bool {
        (*self & *other) == 0
    }
    fn update(&mut self, other: &u64) {
        *self ^= *other;
    }
    fn set(&mut self, idx: i32) {
        *self |= 1u64 << (idx as u64);
    }
    fn unset(&mut self, idx: i32) {
        *self &= !(1u64 << (idx as u64));
    }
    fn lowest_unset_bit(&self) -> i32 {
        (!self).trailing_zeros() as i32
    }
    fn lowest_set_bit(&self) -> i32 {
        self.trailing_zeros() as i32
    }
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

#[derive(Clone)]
pub struct Bitset {
    data: Vec<u64>,
    size: i32,
}

impl Bits for Bitset {
    fn max_capacity() -> i32 {
        i32::max_value()
    }
    fn allocate(size: i32) -> Bitset {
        Bitset {
            data: vec![0u64; ((size + 63) >> 6) as usize],
            size,
        }
    }
    fn disjoint(&self, other: &Bitset) -> bool {
        for i in 0..self.data.len() {
            if (self.data[i] & other.data[i]) != 0 {
                return false;
            }
        }
        true
    }
    fn update(&mut self, other: &Bitset) {
        for i in 0..self.data.len() {
            self.data[i] ^= other.data[i];
        }
    }
    fn set(&mut self, idx: i32) {
        self.data[(idx >> 6) as usize] |= 1u64 << ((idx & 63) as u64);
    }
    fn unset(&mut self, idx: i32) {
        self.data[(idx >> 6) as usize] &= !(1u64 << ((idx & 63) as u64));
    }
    fn lowest_unset_bit(&self) -> i32 {
        for i in 0..self.data.len() {
            let t = (!self.data[i]).trailing_zeros();
            if t != 64 {
                return (t as i32) + ((i as i32) << 6);
            }
        }
        self.size
    }
    fn lowest_set_bit(&self) -> i32 {
        for i in 0..self.data.len() {
            let t = self.data[i].trailing_zeros();
            if t != 64 {
                return (t as i32) + ((i as i32) << 6);
            }
        }
        self.size
    }
    fn is_empty(&self) -> bool {
        for i in 0..self.data.len() {
            if self.data[i] != 0 {
                return false;
            }
        }
        true
    }
}
