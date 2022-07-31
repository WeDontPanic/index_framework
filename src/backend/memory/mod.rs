pub mod backend;
pub mod builder;
pub mod dict;
pub mod postings;
pub mod storage;

use crate::traits::{
    backend::{Backend, NewBackend},
    deser::DeSer,
    dict_item::DictItem,
    dictionary::IndexDictionary,
    postings::IndexPostings,
    storage::IndexStorage,
};
use serde::{Deserialize, Serialize};
use std::{io::Read, marker::PhantomData};

/// Completely in memory index backend
#[derive(Serialize, Deserialize)]
pub struct MemBackend<T, S, Dic, Stor, Post> {
    dict: Dic,
    postings_list: Vec<Post>,
    storage: Stor,
    p: PhantomData<T>,
    p2: PhantomData<S>,
}

impl<T, S, Dic, Stor, Post> Backend<T, S> for MemBackend<T, S, Dic, Stor, Post>
where
    Dic: IndexDictionary<T> + DeSer,
    Stor: IndexStorage<S> + DeSer,
    Post: IndexPostings + DeSer,
    T: DictItem,
    S: DeSer,
{
    type Dict = Dic;
    type Storage = Stor;
    type Postings = Post;

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

    fn posting_count(&self) -> usize {
        self.postings_list.len()
    }

    #[inline]
    fn storage(&self) -> &Self::Storage {
        &self.storage
    }
}

impl<T, S, Dic, Stor, Post> NewBackend<T, S> for MemBackend<T, S, Dic, Stor, Post>
where
    Dic: IndexDictionary<T> + DeSer,
    Stor: IndexStorage<S> + DeSer,
    Post: IndexPostings + DeSer,
    T: DictItem,
    S: DeSer,
{
    #[inline]
    fn new(dict: Self::Dict, postings_list: Vec<Self::Postings>, storage: Self::Storage) -> Self {
        Self {
            dict,
            postings_list,
            storage,
            p: PhantomData,
            p2: PhantomData,
        }
    }
}
