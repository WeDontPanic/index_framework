pub mod default;
pub mod ngram;

use super::Retrieve;

/// Allow custom retrieve algorithms for `Retrieve`
pub trait Retriever<'a, B, T, S>: Iterator<Item = Self::Output> {
    /// Custom output type of retriever
    type Output;

    /// Create a new retriever
    fn new(retr: Retrieve<'a, B, T, S>) -> Self;

    /// Returns the term_ids of the query
    fn q_term_ids(&self) -> &[u32];
}
