use crate::{
    traits::{
        dict_item::DictItem,
        dictionary::{BuildIndexDictionary, IndexDictionary},
    },
    utils::bin_search::generic_binary_search,
};
use compressed_vec::{buffered::BufCVecRef, CVec};
use serde::{Deserialize, Serialize};
use st_file::{traits::IndexedAccess, MemFile};
use std::marker::PhantomData;

/// In memory dictionary
#[derive(Serialize, Deserialize)]
pub struct Dictionary<T> {
    p: PhantomData<T>,
    data: MemFile,
    sort_index: CVec,
}

impl<T: DictItem> Dictionary<T> {
    #[inline]
    pub(crate) fn new() -> Self {
        Self {
            p: PhantomData,
            data: MemFile::new(),
            sort_index: CVec::new(),
        }
    }

    /// Allows pushing multiple items safely
    #[inline]
    pub fn multi_push(&mut self) -> MultInsert<T> {
        MultInsert::new(self)
    }

    /// Inserts an item into the dictionary. `reorder` has to be called afterwards
    pub(crate) fn insert_raw(&mut self, i: T) -> u32 {
        let enc = i.encode_vec();
        let item_id = self.data.insert(&enc);
        assert_eq!(self.data.len() - 1, item_id);
        self.sort_index.push(item_id as u32);
        item_id as u32
    }

    /// Brings the item mapping back in order. Has to be called if changes were made
    /// in the `data` mem-file
    pub(crate) fn reorder(&mut self) {
        let mut vec = self.sort_index.as_vec();
        vec.sort_by(|a, b| {
            let a = self.get_term(*a).unwrap();
            let b = self.get_term(*b).unwrap();
            a.cmp(&b)
        });
        self.sort_index = CVec::from(vec);
    }
}

impl<T: DictItem> IndexDictionary<T> for Dictionary<T> {
    #[inline]
    fn get_id(&self, term: &T) -> Option<u32> {
        let mut buf_reader = BufCVecRef::new(&self.sort_index);
        let res = generic_binary_search((), self.len(), |_, i| {
            let pos = *buf_reader.get_buffered(i).unwrap() as u32;
            let bterm = self.get_term(pos).unwrap();
            (bterm.cmp(term), bterm)
        })
        .ok()?
        .0 as u32;
        Some(self.sort_index.get(res as usize).unwrap())
    }

    #[inline]
    fn get_term(&self, id: u32) -> Option<T> {
        T::decode_vec(self.data.get(id as usize)?)
    }

    #[inline]
    fn len(&self) -> usize {
        self.data.len()
    }
}

impl<T: DictItem> BuildIndexDictionary<T> for Dictionary<T> {
    type Output = Self;

    #[inline]
    fn new() -> Self {
        Self::new()
    }

    #[inline]
    fn insert(&mut self, i: T) -> u32 {
        self.insert_raw(i)
    }

    #[inline]
    fn finish(&mut self) {
        self.reorder();
    }

    #[inline]
    fn build(self) -> Self::Output {
        self
    }
}

pub struct MultInsert<'a, T: DictItem> {
    dict: &'a mut Dictionary<T>,
}

impl<'a, T: DictItem> MultInsert<'a, T> {
    #[inline]
    pub(crate) fn new(dict: &'a mut Dictionary<T>) -> Self {
        Self { dict }
    }

    #[inline]
    pub fn insert(&mut self, item: T) -> u32 {
        self.dict.insert_raw(item)
    }
}

impl<'a, T: DictItem> Drop for MultInsert<'a, T> {
    #[inline]
    fn drop(&mut self) {
        self.dict.reorder();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    pub fn test_dict() {
        let inpdict = &["think", "aal", "auch", "zoo", "make"];

        let mut map: HashMap<&str, u32> = HashMap::new();

        let mut dict = Dictionary::<String>::new();
        {
            let mut push = dict.multi_push();
            for d in inpdict {
                let id = push.insert(d.to_string());
                map.insert(d, id);
            }
        }

        for (d, id) in map {
            assert_eq!(dict.get_id(&d.to_string()).unwrap(), id);
        }

        for a in inpdict {
            assert!(dict.has_term(&a.to_string()));
        }
    }
}
