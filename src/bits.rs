pub trait Bits: Copy {
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
