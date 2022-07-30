use super::file::MemFile;
use crate::traits::{
    deser::DeSer,
    storage::{BuildIndexStorage, IndexStorage},
};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// An in-memory storage for index items
#[derive(Serialize, Deserialize)]
pub struct Storage<S> {
    pub(crate) data: MemFile,
    p: PhantomData<S>,
}

impl<S> Storage<S> {
    #[inline]
    pub(crate) fn new() -> Self {
        Self {
            data: MemFile::new(),
            p: PhantomData,
        }
    }
}

impl<S: DeSer> Storage<S> {
    #[inline]
    pub(crate) fn insert(&mut self, item: S) -> u32 {
        self.data.insert(&item.encode_vec()) as u32
    }
}

impl<S: DeSer> IndexStorage<S> for Storage<S> {
    #[inline]
    fn get_item(&self, id: u32) -> Option<S> {
        S::decode_vec(self.data.get(id as usize)?)
    }

    #[inline]
    fn has_item(&self, id: u32) -> bool {
        (id as usize) < self.data.len()
    }

    #[inline]
    fn len(&self) -> usize {
        self.data.len()
    }
}

impl<S: DeSer> BuildIndexStorage<S> for Storage<S> {
    type Output = Self;

    fn new() -> Self {
        Self::new()
    }

    #[inline]
    fn insert(&mut self, item: S) -> u32 {
        self.insert(item)
    }

    #[inline]
    fn build(self) -> Self::Output {
        self
    }
}