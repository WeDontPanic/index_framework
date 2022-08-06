use std::collections::HashMap;

/// Postings are inverted mappings from a term to all items within the storage which
/// are indexed to this term
pub trait IndexPostings {
    /// Returns the postings-list with a given ID
    fn get_posting(&self, id: u32) -> Vec<u32>;

    /// Returns `true` if the posting storage has a
    /// posting list with the given ID
    fn has_id(&self, id: u32) -> bool;

    fn posting_size(&self, id: u32) -> usize;

    /// Returs the amount of postings
    fn len(&self) -> usize;

    /// Returns an iterator over all postings in the index
    #[inline]
    fn iter(&self) -> PostingIter<Self>
    where
        Self: Sized,
    {
        PostingIter::new(self)
    }

    /// Returs `true` if there is no posting
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait BuildPostings {
    type Output;
    type PostingList;

    fn from_map(map: HashMap<u32, Self::PostingList>) -> Self;

    fn build(self) -> Self::Output;
}

/// An iterator over all posting lists in an index
pub struct PostingIter<'a, P> {
    postings: &'a P,
    pos: usize,
}

impl<'a, P> PostingIter<'a, P> {
    #[inline]
    pub fn new(postings: &'a P) -> Self {
        Self { postings, pos: 0 }
    }
}
impl<'a, P> Iterator for PostingIter<'a, P>
where
    P: IndexPostings,
{
    type Item = Vec<u32>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.postings.len() {
            return None;
        }
        let item = self.postings.get_posting(self.pos as u32);
        self.pos += 1;
        Some(item)
    }
}
