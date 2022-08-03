use crate::traits::{build::ItemMod, storage::IndexStorage};
use compressed_vec::CVec;
use serde::{Deserialize, Serialize};

/// An in-memory index storage for storing u32 compressed
#[derive(Serialize, Deserialize, Default)]
pub struct U32Storage {
    data: CVec,
}

impl U32Storage {
    #[inline]
    pub fn new() -> Self {
        U32Storage { data: CVec::new() }
    }

    #[inline]
    pub fn insert(&mut self, item: u32) -> u32 {
        let id = self.data.len();
        self.data.push(item);
        id as u32
    }
}

impl IndexStorage<u32> for U32Storage {
    #[inline]
    fn get_item(&self, id: u32) -> Option<u32> {
        self.data.get(id as usize)
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

impl ItemMod<u32> for U32Storage {
    #[inline]
    fn set_item(&mut self, id: u32, new: u32) {
        self.data.set(id as usize, new);
    }
}
