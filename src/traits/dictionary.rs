use super::dict_item::DictItem;
use std::marker::PhantomData;

/// Dictionary containing all terms of the index
pub trait IndexDictionary<I: DictItem> {
    /// Returs the ID of the given term
    fn get_id(&self, term: &I) -> Option<u32>;

    /// Returns a term with the given ID
    fn get_term(&self, id: u32) -> Option<I>;

    /// Returns `true` if the Dictionary has the given term
    #[inline]
    fn has_term(&self, term: &I) -> bool {
        self.get_id(term).is_some()
    }

    /// Returs the amount of terms in the dictionary
    fn len(&self) -> usize;

    /// Returns an iterator over all terms in the dictionary
    #[inline]
    fn iter(&self) -> DictIter<Self, I>
    where
        Self: Sized,
    {
        DictIter::new(self)
    }

    /// Returs `true` if there is no item in the dictionary
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait BuildIndexDictionary<I: DictItem> {
    type Output;

    /// Create a new IndexDictionary
    fn new() -> Self;

    /// Inserts a new item into the dict and returns its ID
    fn insert(&mut self, i: I) -> u32;

    /// Called after adding all terms
    fn finish(&mut self) {}

    fn build(self) -> Self::Output;
}

pub struct DictIter<'a, D: IndexDictionary<I>, I: DictItem> {
    inner: &'a D,
    p: PhantomData<I>,
    pos: usize,
}

impl<'a, D, I> DictIter<'a, D, I>
where
    I: DictItem,
    D: IndexDictionary<I>,
{
    #[inline]
    pub(crate) fn new(inner: &'a D) -> Self {
        Self {
            inner,
            p: PhantomData,
            pos: 0,
        }
    }
}

impl<'a, D, I> Iterator for DictIter<'a, D, I>
where
    I: DictItem,
    D: IndexDictionary<I>,
{
    type Item = I;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inner.get_term(self.pos as u32)?;
        self.pos += 1;
        Some(item)
    }
}
