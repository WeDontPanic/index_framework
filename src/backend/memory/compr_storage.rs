use crate::traits::storage::IndexStorage;
use compressed_vec::CVec;
use serde::{Deserialize, Serialize};

/// An in-memory index storage
#[derive(Serialize, Deserialize)]
pub struct Storage {
    data: CVec,
}

impl Storage {
    #[inline]
    pub fn new() -> Self {
        Storage { data: CVec::new() }
    }

    #[inline]
    pub fn insert(&mut self, item: u32) -> u32 {
        let id = self.data.len();
        self.data.push(item);
        id as u32
    }
}

impl IndexStorage<u32> for Storage {
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
