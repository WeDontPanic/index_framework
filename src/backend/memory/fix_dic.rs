use crate::{
    traits::dictionary::{BuildIndexDictionary, IndexDictionary},
    utils::bin_search::generic_binary_search,
    utils::const_arr_deser,
};
use compressed_vec::{buffered::BufCVecRef, CVec};
use serde::{Deserialize, Serialize};

/// String Dictionary with constant term width
#[derive(Serialize, Deserialize)]
pub struct FixDict<const N: usize> {
    #[serde(with = "const_arr_deser")]
    data: Vec<[char; N]>,
    sort_index: CVec,
}

impl<const N: usize> FixDict<N> {
    #[inline]
    pub(crate) fn new() -> Self {
        Self {
            data: vec![],
            sort_index: CVec::new(),
        }
    }

    /// Allows pushing multiple items safely
    #[inline]
    pub fn multi_push(&mut self) -> MultInsert<N> {
        MultInsert::new(self)
    }

    /// Inserts an item into the dictionary. `reorder` has to be called afterwards
    pub(crate) fn insert_raw(&mut self, i: String) -> u32 {
        let id = self.data.len();

        let chars = Self::char_array(&i);

        self.data.push(chars);
        self.sort_index.push(id as u32);
        id as u32
    }

    #[inline]
    fn char_array(i: &str) -> [char; N] {
        i.chars().take(N).collect::<Vec<_>>().try_into().unwrap()
    }

    /// Brings the item mapping back in order. Has to be called if changes were made
    /// in the `data` mem-file
    pub(crate) fn reorder(&mut self) {
        let mut vec = std::mem::take(&mut self.sort_index).as_vec();
        vec.sort_by(|a, b| {
            let a = self.get_term_raw(*a).unwrap();
            let b = self.get_term_raw(*b).unwrap();
            a.cmp(&b)
        });
        self.sort_index = CVec::from(vec);
    }

    #[inline]
    fn get_term_raw(&self, id: u32) -> Option<&[char; N]> {
        self.data.get(id as usize)
    }
}

impl<const N: usize> IndexDictionary<String> for FixDict<N> {
    #[inline]
    fn get_id(&self, term: &String) -> Option<u32> {
        let t_chars = Self::char_array(term);

        let mut buf_read = BufCVecRef::new(&self.sort_index);

        let res = generic_binary_search((), self.len(), |_, i| {
            let pos = *buf_read.get_buffered(i).unwrap() as u32;

            let bterm = self.get_term_raw(pos).unwrap();

            (bterm.cmp(&t_chars), bterm)
        })
        .ok()?
        .0 as u32;
        Some(*buf_read.get_buffered(res as usize).unwrap())
    }

    #[inline]
    fn get_term(&self, id: u32) -> Option<String> {
        let data = self.get_term_raw(id)?;
        Some(data.iter().collect())
    }

    #[inline]
    fn len(&self) -> usize {
        self.data.len()
    }
}

impl<const N: usize> BuildIndexDictionary<String> for FixDict<N> {
    type Output = Self;

    #[inline]
    fn new() -> Self {
        Self::new()
    }

    #[inline]
    fn insert(&mut self, i: String) -> u32 {
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

pub struct MultInsert<'a, const N: usize> {
    dict: &'a mut FixDict<N>,
}

impl<'a, const N: usize> MultInsert<'a, N> {
    #[inline]
    pub(crate) fn new(dict: &'a mut FixDict<N>) -> Self {
        Self { dict }
    }

    #[inline]
    pub fn insert(&mut self, item: String) -> u32 {
        self.dict.insert_raw(item)
    }
}

impl<'a, const N: usize> Drop for MultInsert<'a, N> {
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
        let inpdict = &["tk", "aa", "ch", "oe", "ke"];

        let mut map: HashMap<&str, u32> = HashMap::new();

        let mut dict = FixDict::<2>::new();
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
