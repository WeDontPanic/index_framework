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
    fn map(&mut self, item: u32, terms: &[u32]);

    // Inserts an item into the index and directly maps it
    fn index_item(&mut self, item: S, terms: &[u32]) -> u32 {
        let item_id = self.insert_item(item);
        self.map(item_id, terms);
        item_id
    }

    /// Generate the index
    fn build(self) -> Index<Self::ForBackend, T, S>;
}
