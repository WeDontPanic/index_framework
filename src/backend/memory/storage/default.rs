use crate::traits::{
    build::ItemMod,
    deser::DeSer,
    storage::{BuildIndexStorage, IndexStorage, IndexStorageMod},
};
use serde::{Deserialize, Serialize};
use st_file::{traits::IndexedAccess, MemFile};
use std::marker::PhantomData;

/// An in-memory storage for index items
#[derive(Serialize, Deserialize, Default)]
pub struct Storage<S> {
    data: MemFile,
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

impl<S: DeSer> IndexStorageMod<S> for Storage<S> {
    #[inline]
    fn set_item(&mut self, id: u32, new: S) -> bool{
        self.data.replace(id as usize, &new.encode_vec()).is_some()
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

    #[inline]
    fn get(&self, id: u32) -> Option<S> {
        self.get_item(id)
    }

    #[inline]
    fn len(&self) -> usize {
        self.data.len()
    }
}

impl<S> ItemMod<S> for Storage<S>
where
    S: DeSer,
{
    #[inline]
    fn set_item(&mut self, id: u32, new: S) {
        let encoded = new.encode_vec();
        self.data.replace(id as usize, &encoded);
    }
}
