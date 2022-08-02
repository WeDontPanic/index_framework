use super::{backend::Backend, deser::DeSer, dict_item::DictItem};
use crate::Index;

/// "High-level" interface to allow building indexes
pub trait IndexBuilder<T, S>
where
    T: DictItem,
    S: DeSer,
{
    type ForBackend: Backend<T, S>;

    /// Inserts a new term into the builders dictionary. Returns `Ok(ID)` if the term was added
    /// And Err(ID) if the term already exists.
    fn insert_term(&mut self, term: T) -> Result<u32, u32>;

    /// Inserts a new item into the storage and returns its its ID
    fn insert_item(&mut self, item: S) -> u32;

    // Manually maps an item-id to term-ids in the inverted index
    fn map(&mut self, postings_id: u32, item: u32, terms: &[u32]);

    // Inserts an item into the index and directly maps it
    fn index_new(&mut self, postings_id: u32, item: S, terms: &[u32]) -> u32 {
        let item_id = self.insert_item(item);
        self.map(postings_id, item_id, terms);
        item_id
    }

    // Inserts an item into the index and directly maps it
    fn index_with_terms<I, U>(&mut self, pst_id: u32, item: S, term_iter: I) -> u32
    where
        I: IntoIterator<Item = U>,
        U: Into<T>,
    {
        let item_id = self.insert_item(item);
        let terms = self.terms_to_ids(term_iter);
        self.map(pst_id, item_id, &terms);
        0
    }

    /// Generate the index
    fn build(self) -> Index<Self::ForBackend, T, S>;

    /// Maps an iterator over dict terms to their IDs by new inserting or retrieveng existing terms
    #[inline]
    fn terms_to_ids<I, U>(&mut self, inp: I) -> Vec<u32>
    where
        I: IntoIterator<Item = U>,
        U: Into<T>,
    {
        inp.into_iter()
            .map(|i| self.insert_term(i.into()).unwrap_or_else(|v| v))
            .collect()
    }
}

pub trait ItemMod<T> {
    fn set_item(&mut self, id: u32, new: T);
}
