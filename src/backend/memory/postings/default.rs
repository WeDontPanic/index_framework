use crate::traits::postings::{BuildPostings, IndexPostings};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default)]
pub struct Postings {
    index: HashMap<u32, Vec<u32>>,
}

impl Postings {
    #[inline]
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
        }
    }

    #[inline]
    pub fn from_map(map: HashMap<u32, Vec<u32>>) -> Self {
        Self { index: map }
    }

    #[inline]
    fn get(&self, id: u32) -> Option<&Vec<u32>> {
        self.index.get(&id)
    }
}

impl IndexPostings for Postings {
    #[inline]
    fn get_posting(&self, id: u32) -> Vec<u32> {
        self.get(id).cloned().unwrap_or_default()
    }

    #[inline]
    fn has_id(&self, id: u32) -> bool {
        self.get(id).is_some()
    }

    fn posting_size(&self, id: u32) -> usize {
        self.get(id).map(|i| i.len()).unwrap_or(0)
    }

    #[inline]
    fn len(&self) -> usize {
        self.index.len()
    }
}

impl BuildPostings for Postings {
    type Output = Self;
    type PostingList = Vec<u32>;

    #[inline]
    fn from_map(map: HashMap<u32, Self::PostingList>) -> Self {
        Self::from_map(map)
    }

    #[inline]
    fn build(self) -> Self::Output {
        self
    }
}
