pub mod iter;

use crate::{
    traits::{
        backend::Backend, deser::DeSer, dict_item::DictItem, dictionary::IndexDictionary,
        postings::IndexPostings,
    },
    Index,
};
use iter::RetrieveIter;

/// Retrieves stuff from an index
pub struct Retrieve<'a, B, T, S> {
    index: &'a Index<B, T, S>,
    limit: usize,
    unique: bool,
    terms: Vec<u32>,
    postings: Vec<u32>,
}

impl<'a, B, T, S> Retrieve<'a, B, T, S>
where
    B: Backend<T, S>,
    T: DictItem,
    S: DeSer,
    <<B as Backend<T, S>>::Postings as IndexPostings>::List: IntoIterator<Item = u32>,
{
    #[inline]
    pub(crate) fn new(index: &'a Index<B, T, S>) -> Self {
        Self {
            index,
            unique: false,
            limit: 0,
            terms: vec![],
            postings: vec![0],
        }
    }

    /// Shortcut for `.into_iter()`
    #[inline]
    pub fn get(self) -> RetrieveIter<'a, B, T, S> {
        RetrieveIter::new(self)
    }

    /// Collects all items and returns them in a new vec
    #[inline]
    pub fn get_all(self) -> Vec<S> {
        RetrieveIter::new(self).collect()
    }

    #[inline]
    pub fn unique(mut self) -> Self {
        self.unique = true;
        self
    }

    #[inline]
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    #[inline]
    pub fn all(mut self) -> Self {
        self.limit = 0;
        self
    }

    #[inline]
    pub fn by_term<U>(mut self, term: U) -> Self
    where
        U: Into<T>,
    {
        let id = self.index.dict().get_id(term);
        if let Some(id) = id {
            self.terms = vec![id];
        }
        self
    }

    #[inline]
    pub fn by_terms<I, U>(mut self, terms: I) -> Self
    where
        I: IntoIterator<Item = U>,
        U: Into<T>,
    {
        self.terms = terms
            .into_iter()
            .filter_map(|i| self.index.dict().get_id(i))
            .collect();
        self
    }

    #[inline]
    pub fn by_term_id(mut self, t_id: u32) -> Self {
        if self.index.dict().has_term_id(t_id) {
            self.terms.push(t_id);
        }
        self
    }

    #[inline]
    pub fn by_term_ids<I>(mut self, t_ids: I) -> Self
    where
        I: IntoIterator<Item = u32>,
    {
        self.terms = t_ids
            .into_iter()
            .filter(|i| self.index.dict().has_term_id(*i))
            .collect();
        self
    }

    #[inline]
    pub fn add_term<U>(mut self, term: U) -> Self
    where
        U: Into<T>,
    {
        let id = self.index.dict().get_id(term);
        if let Some(id) = id {
            self.terms.push(id);
        }
        self
    }

    #[inline]
    pub fn add_terms<I, U>(mut self, terms: I) -> Self
    where
        I: IntoIterator<Item = U>,
        U: Into<T>,
    {
        let iter = terms
            .into_iter()
            .filter_map(|i| self.index.dict().get_id(i));
        self.terms.extend(iter);
        self
    }

    #[inline]
    pub fn add_term_id(mut self, t_id: u32) -> Self {
        if self.index.dict().has_term_id(t_id) {
            self.terms.push(t_id);
        }
        self
    }

    #[inline]
    pub fn add_term_ids<I>(mut self, t_ids: I) -> Self
    where
        I: IntoIterator<Item = u32>,
    {
        let iter = t_ids
            .into_iter()
            .filter(|i| self.index.dict().has_term_id(*i));
        self.terms.extend(iter);
        self
    }

    #[inline]
    pub fn in_postings<I>(mut self, p: I) -> Self
    where
        I: IntoIterator<Item = u32>,
    {
        self.postings = p.into_iter().collect();
        self
    }
}

impl<'a, B, T, S> IntoIterator for Retrieve<'a, B, T, S>
where
    B: Backend<T, S>,
    T: DictItem,
    S: DeSer,
    <<B as Backend<T, S>>::Postings as IndexPostings>::List: IntoIterator<Item = u32>,
{
    type Item = S;
    type IntoIter = RetrieveIter<'a, B, T, S>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        RetrieveIter::new(self)
    }
}
