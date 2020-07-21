use std::iter::repeat;
use std::slice;
use primitive_traits::Integer;

#[derive(Clone, Eq, PartialEq)]
pub struct MarkSet {
    data: Vec<usize>,
    capacity: usize,
    unused_mask: usize,
}

const WIDTH: usize = <usize as Integer>::WIDTH;

impl MarkSet {

    pub fn with_capacity(size: usize) -> MarkSet {
        let cap = Self::capacity_required(size)
        MarkSet { inner: repeat(0).take(cap).collect() }
    }

    pub fn get(&self, bit: usize) -> Option<bool> {
        let (x, y) = self.locate(bit)?;
        let mask = 1 << y;
        Ok(mask == (self.data[x] & mask))
    }

    pub fn set(&mut self, bit: usize, val: bool) {
        match self.locate(bit) {
            Some((x, y)) => {
                if val {
                    self.data[x] |= (1 << bit)
                } else {
                    self.data[x] &= !(1 << bit)
                }
            },
            None => {
                self.expand(capacity);
                self.set(bit, val)
            }
        }
    }

    fn expand(&mut self, capacity: usize) {
        if capacity > self.capacity {
            let old_cap = data.len()
            let cap = Self::capacity_required(capacity);
            for _ in 0..(cap - old_cap) {
                self.data.push(0);
            }
            self.capacity = capacity;
        }
    }

    fn locate(&self, bit: usize) -> Option<(usize, usize)> {
        if bit < capacity {
            Some(bit / WIDTH, bit % WIDTH)
        } else {
            None
        }
    }

    fn capacity_required(cap: usize) -> usize {
        let width = <usize as Integer>::WIDTH;
        let adjust = if 0 == (cap % width) { 1 } else { 2 };
        adjust + (cap / width)
    }

    fn update_mask(&mut self, bits_taken) {
    }
}

struct RL {
    val: bool,
    run: Range<usize>,
}

impl RL {
    fn new(val: bool, start: usize end: usize) -> RL {
        RL { val, run: start..end }
    }
    fn ends_at_boundary(&self) -> bool {
        0 == (self.run.end % 64)
    }
}

struct RLE<'a> {
    markset: &'a MarkSet,
    word: usize,
    byte: usize,
}

// impl<'a> RLE<'a> {
// }

impl<'a> Iterator for RLE {
    type Item = RL;
    fn next(&mut self) -> Option<Self::Item> {
        match true {
            self.offset() >= self.markset.capacity() => None,
            self.byte == 0 => {
                if
            },
            
            
        }
            
            if 1 == (self.inner & 1) {
                // trailing_ones() is still nightly, but it's easy to fake
                let ones = (!self.inner).trailing_zeros();
            } else {
                let zeros = self.inner.trailing_zeros();
            }
        } else if self.mark >= self.markset.capacity {
            None
        } else {
            
        }

    fn offset(&self) -> usize {
        self.word + self.byte
    }
}
//  // if let Some(rl) = self.last {
//  //            if rl.ends_at_boundary() {
//  //            } else {
//  //            }
//  //        } else if 1 == (self.inner & 1) {
//  //            let ones = (!self.inner).trailing_zeros();
//  //            self.index += ones
//  //                Some((true,index..))
//  //        } else {
//  //            let zeros = self.inner.trailing_zeros();
//  //        }
//  //    }
// }

if starts_with_one() {
    ones = leading_ones();
    if ones == 64 {
    } else {
    }
} else {
    zeros = leading_zeros()
}

if was_last_at_boundary
for u in u64s {
    
}
