use crate::traits::postings::{BuildPostings, IndexPostings};
use compressed_vec::{buffered::BufCVecRef, CVec};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default)]
pub struct Postings {
    /// Maps dimension indexes to positions in `data`
    index: CVec,
    /// Contains the vector ids for each dimension
    data: CVec,
}

impl Postings {
    #[inline]
    pub fn new() -> Self {
        Self {
            index: CVec::new(),
            data: CVec::new(),
        }
    }

    #[inline]
    pub(crate) fn from_map(map: HashMap<u32, Vec<u32>>) -> Self {
        let mut index = CVec::new();
        let mut data = CVec::new();

        let mut prev_id: Option<u32> = None;

        let first = *map.iter().map(|i| i.0).min().unwrap();
        for _ in 0..first {
            index.push(0);
        }

        for (term_id, item_ids) in map.into_iter().sorted_by(|a, b| a.0.cmp(&b.0)) {
            if prev_id.is_none() {
                prev_id = Some(term_id);
            }

            // Fill non mapped dimensions with 0s to make the CVS replace a HashMap
            let ld = prev_id.as_ref().unwrap();
            for _ in ld + 1..term_id {
                index.push(data.len() as u32);
                data.push(0);
            }

            // Push index indice
            index.push(data.len() as u32);

            // Push data
            data.push(item_ids.len() as u32);
            data.extend(item_ids);

            prev_id = Some(term_id);
        }

        Self { index, data }
    }

    #[inline]
    fn get(&self, id: u32) -> Option<Vec<u32>> {
        let arr_start = self.index.get(id as usize)? as usize;

        let mut buf_vec = BufCVecRef::new(&self.data);

        // Length of following vec containing the vector IDs
        let arr_len = *buf_vec.get_buffered(arr_start)? as usize;

        // Padded values have a length of 0
        if arr_len == 0 {
            return None;
        }

        let mut out = Vec::with_capacity(arr_len);

        // Take all elements of array. `arr_len` contains the count of all items in the array
        for pos in (arr_start + 1)..(arr_start + arr_len + 1) {
            out.push(*buf_vec.get_buffered(pos as usize)?);
        }

        Some(out)
    }
}

impl IndexPostings for Postings {
    #[inline]
    fn get_posting(&self, id: u32) -> Vec<u32> {
        self.get(id).unwrap_or_default()
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
