use super::{
    deser::DeSer, dict_item::DictItem, dictionary::IndexDictionary, postings::IndexPostings,
    storage::IndexStorage,
};
use std::path::Path;

/// Defines how the index handles the data internally by having to specify type implementations
/// for all traits an index requires to work properly
pub trait Backend<T, S>
where
    T: DictItem,
    S: DeSer,
{
    type Dict: IndexDictionary<T>;
    type Postings: IndexPostings;
    type Storage: IndexStorage<S>;

    type OpenError;

    /// Opens an index backend
    fn open<P: AsRef<Path>>(file: P) -> Result<Self, Self::OpenError>
    where
        Self: Sized;

    fn encode(&self) -> Vec<u8>;

    fn dict(&self) -> &Self::Dict;
    fn postings(&self) -> &Self::Postings;
    fn storage(&self) -> &Self::Storage;

    /// Returns `true` if the index doesn't contain index data
    #[inline]
    fn is_empty(&self) -> bool {
        self.dict().is_empty() || self.postings().is_empty() || self.storage().is_empty()
    }
}

pub trait NewBackend<T, S>: Backend<T, S>
where
    T: DictItem,
    S: DeSer,
{
    fn new(dict: Self::Dict, postings: Self::Postings, storage: Self::Storage) -> Self;
}
