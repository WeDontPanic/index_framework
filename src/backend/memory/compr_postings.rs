use crate::traits::postings::{BuildPostings, IndexPostings};
use compressed_vec::{buffered::BufCVecRef, CVec};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Postings {
    /// Maps dimension indexes to positions in `data`
    index: CVec,
    /// Contains the vector ids for each dimension
    data: CVec,
}

impl Postings {
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

        let mut prev_term_id: Option<u32> = None;

        for (dim, item_ids) in map.into_iter().sorted_by(|a, b| a.0.cmp(&b.0)) {
            if prev_term_id.is_none() {
                prev_term_id = Some(dim);
            }

            if prev_term_id.as_ref().unwrap() + 1 < dim {
                panic!("Invalid index");
            }

            // Push index indice
            index.push(data.len() as u32);

            // Push data
            data.push(item_ids.len() as u32);
            data.extend(item_ids);

            prev_term_id = Some(dim);
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
        for pos in (arr_start + 1)..(arr_start + arr_len) {
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

    #[inline]
    fn from_map(map: HashMap<u32, Vec<u32>>) -> Self {
        Self::from_map(map)
    }

    #[inline]
    fn build(self) -> Self::Output {
        self
    }
}
