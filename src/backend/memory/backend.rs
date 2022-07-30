use super::{compr_postings::Postings, dict::Dictionary, storage::Storage};
use crate::traits::{
    backend::{Backend, NewBackend},
    deser::DeSer,
    dict_item::DictItem,
};
use serde::{Deserialize, Serialize};
use std::io::Read;

/// Completely in memory index backend
#[derive(Serialize, Deserialize)]
pub struct MemoryBackend<D, S> {
    dict: Dictionary<D>,
    postings_list: Vec<Postings>,
    storage: Storage<S>,
}

impl<D, S> Backend<D, S> for MemoryBackend<D, S>
where
    D: DictItem,
    S: DeSer,
{
    type Dict = Dictionary<D>;
    type Postings = Postings;
    type Storage = Storage<S>;

    fn decode_from<R: Read>(reader: R) -> Option<Self>
    where
        Self: Sized,
    {
        bincode::deserialize_from(reader).ok()
    }

    #[inline]
    fn encode(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    #[inline]
    fn dict(&self) -> &Self::Dict {
        &self.dict
    }

    #[inline]
    fn postings(&self, id: u32) -> Option<&Self::Postings> {
        self.postings_list.get(id as usize)
    }

    #[inline]
    fn storage(&self) -> &Self::Storage {
        &self.storage
    }

    fn posting_count(&self) -> usize {
        self.postings_list.len()
    }
}

impl<D, S> NewBackend<D, S> for MemoryBackend<D, S>
where
    D: DictItem,
    S: DeSer,
{
    #[inline]
    fn new(dict: Self::Dict, postings_list: Vec<Self::Postings>, storage: Self::Storage) -> Self {
        Self {
            dict,
            postings_list,
            storage,
        }
    }
}
