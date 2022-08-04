pub mod retriever;

use crate::{
    traits::{
        backend::Backend, deser::DeSer, dict_item::DictItem, dictionary::IndexDictionary,
        postings::IndexPostings,
    },
    Index,
};

use retriever::Retriever;

/// Retrieves stuff from an index
#[derive(Clone)]
pub struct Retrieve<'a, B, T, S> {
    index: &'a Index<B, T, S>,
    limit: usize,
    unique: bool,
    terms: Vec<u32>,
    posting_ids: Vec<u32>,
}

impl<'a, B, T, S> Retrieve<'a, B, T, S>
where
    B: Backend<T, S>,
    T: DictItem,
    S: DeSer,
    <B as Backend<T, S>>::Postings: IndexPostings<List = Vec<u32>>,
{
    #[inline]
    pub(crate) fn new(index: &'a Index<B, T, S>) -> Self {
        Self {
            index,
            unique: false,
            limit: 0,
            terms: vec![],
            posting_ids: vec![0],
        }
    }

    /// Retrieve results
    #[inline]
    #[must_use = "Output is lazy"]
    pub fn get<R>(self) -> R
    where
        R: Retriever<'a, B, T, S>,
    {
        R::new(self)
    }

    /// Collects all items and returns them in a new vec
    #[inline]
    pub fn get_all<R>(self) -> Vec<R::Output>
    where
        R: Retriever<'a, B, T, S>,
    {
        self.get::<R>().collect()
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
    pub fn in_posting<I>(mut self, p: u32) -> Self
    where
        I: IntoIterator<Item = u32>,
    {
        self.posting_ids = vec![p];
        self
    }

    #[inline]
    pub fn in_postings<I>(mut self, p: I) -> Self
    where
        I: IntoIterator<Item = u32>,
    {
        self.posting_ids = p.into_iter().collect();
        self
    }
}
