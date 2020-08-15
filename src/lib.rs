#![no_std]
extern crate alloc;

use alloc::vec::Vec;
use core::convert::From;
use core::future::Future;
use core::iter::repeat;
use core::pin::Pin;
use futures_core::stream::Stream;
use rle_bitset::*;

use core::task::{Context, Poll};

/// A Stream of results from Futures polled as they arrive, out of order.
pub struct Many<F> {
    futures: Vec<F>,
    done: Vec<usize>, // Bitset for whether a future is done.
    remaining: usize,
}

impl<F: Future> Many<F> {

    /// Creates a new [`crate::Many`].
    pub fn new() -> Self {
        Many { futures: Vec::new(), done: Vec::new(), remaining: 0 }
    }
    
    /// Adds a new Future to be polled.
    pub fn push(&mut self, f: F) {
        self.futures.push(f);
        self.remaining += 1;
        if words_needed(self.remaining) != self.done.len() {
            self.done.push(0);
        }
    }
}

impl<F: Future> From<Vec<F>> for Many<F> {
    fn from(futures: Vec<F>) -> Self {
        let remaining = futures.len();
        let done = repeat(0).take(words_needed(remaining)).collect();
        Many { futures, done, remaining }
    }
}

impl<F: Future> Stream for Many<F> {
    type Item = F::Output;
    fn poll_next(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Option<F::Output>> {
        let this = unsafe { self.get_unchecked_mut() };
        if this.remaining == 0 { // Nothing to do!
            Poll::Ready(None)
        } else if this.remaining == this.futures.len() {
            // Until a future has completed, we can avoid bit faffing.
            for (i, f) in this.futures.iter_mut().enumerate() {
                let pin = unsafe { Pin::new_unchecked(f) }; 
                if let Poll::Ready(v) = pin.poll(ctx) {
                    this.done.set_bit(i, true).unwrap();
                    this.remaining -= 1;
                    return Poll::Ready(Some(v));
                }
            }
            Poll::Pending
        } else {
            // At least one future has completed, so we have to check bits.
            for rl in this.done.run_lengths(..this.futures.len()).unwrap() {
                if !rl.value {
                    for i in rl.run {
                        let pin = unsafe { Pin::new_unchecked(&mut this.futures[i]) };
                        if let Poll::Ready(v) = pin.poll(ctx) {
                            this.done.set_bit(i, true).unwrap();
                            this.remaining -= 1;
                            return Poll::Ready(Some(v));
                        }
                    }
                }
            }
            Poll::Pending
        }
    }
}
