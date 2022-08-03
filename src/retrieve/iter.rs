use std::collections::HashSet;

use super::Retrieve;
use crate::{
    traits::{
        backend::Backend, deser::DeSer, dict_item::DictItem, postings::IndexPostings,
        storage::IndexStorage,
    },
    Index,
};

/// Iterator over results of a retrieve query
pub struct RetrieveIter<'a, B, T, S> {
    retrieve: Retrieve<'a, B, T, S>,
    storage_buf: Vec<u32>,
    seen: HashSet<u32>,
}

impl<'a, B, T, S> RetrieveIter<'a, B, T, S>
where
    B: Backend<T, S>,
    T: DictItem,
    S: DeSer,
    <<B as Backend<T, S>>::Postings as IndexPostings>::List: IntoIterator<Item = u32>,
{
    #[inline]
    pub(crate) fn new(retrieve: Retrieve<'a, B, T, S>) -> Self {
        Self {
            retrieve,
            storage_buf: Vec::with_capacity(10),
            seen: HashSet::new(),
        }
    }

    #[inline]
    fn index(&self) -> &Index<B, T, S> {
        &self.retrieve.index
    }

    fn get_or_fill(&mut self) -> Option<&mut Vec<u32>> {
        if self.storage_buf.is_empty() {
            self.fill_buff()?;
        }

        assert!(!self.storage_buf.is_empty());

        return Some(&mut self.storage_buf);
    }

    /// Fills the iterators buff with new storage IDs from the index.
    /// Returns `None` if there is nothing left to add. Only countains
    /// Item IDs that weren't seen before
    fn fill_buff(&mut self) -> Option<()> {
        debug_assert!(self.storage_buf.is_empty());

        loop {
            let t_id = self.retrieve.terms.pop()?;

            for post_id in &self.retrieve.postings {
                if let Some(postings) = self.index().postings(*post_id) {
                    let iter = postings.get_posting(t_id).into_iter().filter(|i| {
                        if !self.retrieve.unique {
                            return true;
                        }
                        if self.seen.contains(i) {
                            return false;
                        }
                        self.seen.insert(*i);
                        true
                    });
                    self.storage_buf.extend(iter);
                }
            }

            // Done when added something to the buf
            if !self.storage_buf.is_empty() {
                break;
            }
        }

        Some(())
    }
}

impl<'a, B, T, S> Iterator for RetrieveIter<'a, B, T, S>
where
    B: Backend<T, S>,
    T: DictItem,
    S: DeSer,
    <<B as Backend<T, S>>::Postings as IndexPostings>::List: IntoIterator<Item = u32>,
{
    type Item = S;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let next_id = self.get_or_fill()?.pop()?;
        let item = self
            .index()
            .storage()
            .get_item(next_id)
            .expect("Invalid index");
        Some(item)
    }
}
