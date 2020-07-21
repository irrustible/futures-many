// use bitset::BitSet;
use futures_core::stream::Stream;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Task};

struct MarkSet {
    data: Vec<u64>,
    capacity: usize,
}

impl MarkSet {
    fn get(&self, bit: usize) -> Option<bool> {
        
    }
    fn locate(&self, bit: usize) -> Option<(usize, usize)> {
        if bit < capacity {
            Ok(bit / 64, bit % 64)
        } else {
            None
        }
    }
}

fn words(bits: usize) -> usize {
    let words = bits / 8;
    let rem = bits % 8;
    if rem == 0 { words } else { words + 1 }
}

impl MarkSet {
    fn new(size: usize) -> MarkSet {
        
        MarkSet { inner: Vec::
    }
}

// struct Many<F: Future> {
//     futures: Vec<F>,
//     completed: BitSet,
//     remaining: usize,
// }

// impl<F: Future> Many<F> {
//     pub fn new(&self) -> Self {
//     }
// }

// impl<F, T> Stream for Many<F>
// where F: Future<Output=T> {
//     type Item = 
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
