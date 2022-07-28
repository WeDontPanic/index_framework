use super::{compr_postings::Postings, dict::Dictionary, storage::Storage};
use crate::traits::{
    backend::{Backend, NewBackend},
    deser::DeSer,
    dict_item::DictItem,
};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader, path::Path};

/// Completely in memory index backend
#[derive(Serialize, Deserialize)]
pub struct MemoryBackend<D, S> {
    dict: Dictionary<D>,
    postings: Postings,
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
    type OpenError = ();

    fn open<P: AsRef<Path>>(file: P) -> Result<Self, Self::OpenError>
    where
        Self: Sized,
    {
        let r = BufReader::new(File::open(file).map_err(|_| ())?);
        bincode::deserialize_from(r).map_err(|_| ())
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
    fn postings(&self) -> &Self::Postings {
        &self.postings
    }

    #[inline]
    fn storage(&self) -> &Self::Storage {
        &self.storage
    }
}

impl<D, S> NewBackend<D, S> for MemoryBackend<D, S>
where
    D: DictItem,
    S: DeSer,
{
    #[inline]
    fn new(dict: Self::Dict, postings: Self::Postings, storage: Self::Storage) -> Self {
        Self {
            dict,
            postings,
            storage,
        }
    }
}
