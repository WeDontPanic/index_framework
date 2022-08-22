use crate::traits::storage::{BuildIndexStorage, IndexStorage};
use serde::{Deserialize, Serialize};

/// An in-memory storage for index items
#[derive(Serialize, Deserialize, Default)]
pub struct Storage {
    len: u32,
}

impl Storage {
    #[inline]
    pub(crate) fn new() -> Self {
        Self { len: 0 }
    }
}

impl Storage {
    #[inline]
    pub(crate) fn insert(&mut self, item: u32) -> u32 {
        let id = self.len;
        self.len += 1;
        assert_eq!(item, id);
        id
    }
}

impl IndexStorage<u32> for Storage {
    #[inline]
    fn get_item(&self, id: u32) -> Option<u32> {
        self.has_item(id).then(|| id)
    }

    #[inline]
    fn has_item(&self, id: u32) -> bool {
        id < self.len
    }

    #[inline]
    fn len(&self) -> usize {
        self.len as usize
    }
}

impl BuildIndexStorage<u32> for Storage {
    type Output = Self;

    fn new() -> Self {
        Self::new()
    }

    #[inline]
    fn insert(&mut self, item: u32) -> u32 {
        self.insert(item)
    }

    #[inline]
    fn build(self) -> Self::Output {
        self
    }

    #[inline]
    fn get(&self, id: u32) -> Option<u32> {
        self.get_item(id)
    }

    #[inline]
    fn len(&self) -> usize {
        self.len as usize
    }
}
