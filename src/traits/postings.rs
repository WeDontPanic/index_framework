use std::collections::HashMap;

/// Postings are inverted mappings from a term to all items within the storage which
/// are indexed to this term
pub trait IndexPostings {
    fn get_posting(&self, id: u32) -> Vec<u32>;
    fn has_id(&self, id: u32) -> bool;
    fn posting_size(&self, id: u32) -> usize;

    /// Returs the amount of postings
    fn len(&self) -> usize;

    /// Returs `true` if there is no posting
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait BuildPostings {
    type Output;

    fn from_map(map: HashMap<u32, Vec<u32>>) -> Self;

    fn build(self) -> Self::Output;
}
