use super::Retriever;
use crate::{
    retrieve::Retrieve,
    traits::{
        backend::Backend, deser::DeSer, dict_item::DictItem, postings::IndexPostings,
        storage::IndexStorage,
    },
    utils::lock_step::LockStepIter,
    Index,
};
use order_struct::OrderBy;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

/// NGram optimized retriever
pub struct NGramRetriever<'a, const N: usize, B, T, S> {
    retrieve: Retrieve<'a, B, T, S>,
    item_ids: Vec<u32>,
    did_setup: bool,
}

impl<'a, const N: usize, B, T, S> Retriever<'a, B, T, S> for NGramRetriever<'a, N, B, T, S>
where
    B: Backend<T, S>,
    T: DictItem,
    S: DeSer,
    <B as Backend<T, S>>::Postings: IndexPostings<List = Vec<u32>>,
{
    type Output = S;

    #[inline]
    fn new(mut retr: Retrieve<'a, B, T, S>) -> Self {
        retr.terms.sort_unstable();
        Self {
            retrieve: retr,
            item_ids: vec![],
            did_setup: false,
        }
    }

    #[inline]
    fn q_term_ids(&self) -> &[u32] {
        &self.retrieve.terms
    }
}

impl<'a, const N: usize, B, T, S> NGramRetriever<'a, N, B, T, S>
where
    B: Backend<T, S>,
    T: DictItem,
    S: DeSer,
    <B as Backend<T, S>>::Postings: IndexPostings<List = Vec<u32>>,
{
    /// Loads all required stuff for the iterator. Returns `None` if there is no such
    fn setup(&mut self) -> Option<()> {
        self.did_setup = true;

        // Posting lists for all input terms
        let term_posts = self.make_terms_posings();

        // Map of StorageItemID -> MatchingTermCount
        let itm_post_freqs = Self::calc_post_freqs(&term_posts);

        // Storage Item ids
        self.item_ids = Self::max_n(itm_post_freqs, self.retrieve.limit);

        // Return None if empty
        (!self.item_ids.is_empty()).then(|| ())?;

        Some(())
    }

    #[inline]
    fn make_terms_posings(&self) -> Vec<Vec<u32>> {
        self.retrieve
            .terms
            .iter()
            .flat_map(|i| {
                self.retrieve.posting_ids.iter().filter_map(|pid| {
                    let postings = self.index().postings(*pid)?.get_posting(*i);
                    (!postings.is_empty()).then(|| postings)
                })
            })
            .collect()
    }

    fn calc_post_freqs(term_posts: &[Vec<u32>]) -> HashMap<u32, u32> {
        let mut id_count: HashMap<u32, u32> = HashMap::new();

        let mut seen: HashSet<u32> = HashSet::new();
        let mut added = vec![];

        for (pos, postings) in term_posts.iter().enumerate() {
            for i in postings.iter() {
                if !seen.contains(i) {
                    id_count.insert(*i, 1);
                    added.push(*i);
                }
            }

            for list in term_posts.iter().skip(pos + 1) {
                let lsi = LockStepIter::new(postings.iter(), list.iter());
                for i in lsi.filter(|i| !seen.contains(*i)) {
                    *id_count.entry(*i).or_default() += 1;
                }
            }

            seen.extend(added.drain(..));
        }

        id_count
    }

    /// Gets first n HashMap keys by highest map-value first
    fn max_n(inp: HashMap<u32, u32>, n: usize) -> Vec<u32> {
        let mut bin_heap = BinaryHeap::with_capacity(inp.len());
        for (k, v) in inp.iter() {
            bin_heap.push(OrderBy::new((*k, *v), |a, b| {
                let cmp = a.1.cmp(&b.1).reverse();

                // keep a persistent order
                if cmp == Ordering::Equal {
                    return a.0.cmp(&b.0).reverse();
                }

                cmp.reverse()
            }));
        }

        let mut len = bin_heap.len();
        if n > 0 {
            len = len.min(n);
        }
        let mut vec = Vec::with_capacity(len);
        vec.extend((0..len).map(|_| bin_heap.pop().unwrap().into_inner().0));

        vec
    }

    #[inline]
    fn index(&self) -> &Index<B, T, S> {
        self.retrieve.index
    }
}

impl<'a, const N: usize, B, T, S> Iterator for NGramRetriever<'a, N, B, T, S>
where
    B: Backend<T, S>,
    T: DictItem,
    S: DeSer,
    <B as Backend<T, S>>::Postings: IndexPostings<List = Vec<u32>>,
{
    type Item = S;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if !self.did_setup {
            self.setup()?;
        }

        let item_id = self.item_ids.pop()?;
        Some(self.index().storage().get_item(item_id).unwrap())
    }
}
