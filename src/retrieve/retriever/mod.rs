pub mod default;

use super::Retrieve;

/// Allow custom retrieve algorithms for `Retrieve`
pub trait Retriever<'a, B, T, S>: Iterator<Item = S> {
    fn new(retr: Retrieve<'a, B, T, S>) -> Self;
}
