use crate::{
    traits::{
        backend::{Backend, NewBackend},
        build::IndexBuilder,
        deser::DeSer,
        dict_item::DictItem,
        dictionary::BuildIndexDictionary,
        postings::{BuildPostings, IndexPostings},
        storage::BuildIndexStorage,
    },
    Index,
};
use std::{collections::HashMap, hash::Hash, marker::PhantomData};

/// Generic builder for memory indexes
pub struct MemIndexBuilder<B, T, S, DD, SS, PP> {
    pub dict: DD,
    pub storage: SS,
    pub postings: HashMap<u32, Vec<u32>>,
    pub term_map: HashMap<T, u32>,
    p: PhantomData<S>,
    b: PhantomData<B>,
    pp: PhantomData<PP>,
}

impl<B, T, S, DD, SS, PP> MemIndexBuilder<B, T, S, DD, SS, PP>
where
    B: Backend<T, S>,
    T: DictItem,
    S: DeSer,
    DD: BuildIndexDictionary<T, Output = B::Dict>,
    SS: BuildIndexStorage<S, Output = B::Storage>,
    PP: BuildPostings<Output = B::Postings, PostingList = <B::Postings as IndexPostings>::List>,
{
    #[inline]
    pub fn new() -> Self {
        let dict = DD::new();
        let storage = SS::new();
        let postings = HashMap::new();
        let term_map = HashMap::new();
        Self {
            dict,
            storage,
            postings,
            term_map,
            p: PhantomData,
            b: PhantomData,
            pp: PhantomData,
        }
    }

    #[inline]
    pub fn dict(&self) -> &DD {
        &self.dict
    }

    #[inline]
    pub fn storage(&self) -> &SS {
        &self.storage
    }

    #[inline]
    pub fn postings(&self) -> &HashMap<u32, Vec<u32>> {
        &self.postings
    }

    #[inline]
    pub fn term_map(&self) -> &HashMap<T, u32> {
        &self.term_map
    }

    #[inline]
    pub fn postings_mut(&mut self) -> &mut HashMap<u32, Vec<u32>> {
        &mut self.postings
    }
}

impl<B, T, S, DD, SS, PP> IndexBuilder<T, S> for MemIndexBuilder<B, T, S, DD, SS, PP>
where
    B: Backend<T, S> + NewBackend<T, S>,
    T: DictItem + Hash + Clone,
    S: DeSer,
    DD: BuildIndexDictionary<T, Output = B::Dict>,
    SS: BuildIndexStorage<S, Output = B::Storage>,
    PP: BuildPostings<Output = B::Postings, PostingList = <B::Postings as IndexPostings>::List>,
    <<B as Backend<T, S>>::Postings as IndexPostings>::List: FromIterator<u32>,
{
    type ForBackend = B;

    fn insert_term(&mut self, term: T) -> Result<u32, u32> {
        if let Some(id) = self.term_map.get(&term) {
            return Err(*id);
        }

        let id = self.dict.insert(term.clone());

        self.term_map.insert(term, id);

        Ok(id)
    }

    #[inline]
    fn insert_item(&mut self, item: S) -> u32 {
        self.storage.insert(item)
    }

    #[inline]
    fn map(&mut self, item: u32, terms: &[u32]) {
        for term in terms {
            self.postings.entry(*term).or_default().push(item);
        }
    }

    fn build(mut self) -> Index<Self::ForBackend, T, S> {
        self.dict.finish();

        let postings: HashMap<_, _> = self
            .postings
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect();

        let postings = PP::from_map(postings).build();
        let dict = self.dict.build();
        let storage = self.storage.build();

        Index::new(B::new(dict, postings, storage))
    }
}
