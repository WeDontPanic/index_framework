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
    pub postings_list: Vec<HashMap<u32, Vec<u32>>>,
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
    pub fn new(postings_len: usize) -> Self {
        if postings_len < 1 {
            panic!("At least one postings required!");
        }

        let dict = DD::new();
        let storage = SS::new();
        let postings_list: Vec<_> = (0..postings_len).map(|_| HashMap::new()).collect();
        let term_map = HashMap::new();
        Self {
            dict,
            storage,
            postings_list,
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
    pub fn postings(&self, id: usize) -> Option<&HashMap<u32, Vec<u32>>> {
        self.postings_list.get(id)
    }

    #[inline]
    pub fn term_map(&self) -> &HashMap<T, u32> {
        &self.term_map
    }

    #[inline]
    pub fn postings_mut(&mut self, pos: usize) -> Option<&mut HashMap<u32, Vec<u32>>> {
        self.postings_list.get_mut(pos)
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
    fn map(&mut self, postings_id: u32, item: u32, terms: &[u32]) {
        let postings = self
            .postings_list
            .get_mut(postings_id as usize)
            .expect("Postings with ID {postings_id} not setup!");
        for term in terms {
            postings.entry(*term).or_default().push(item);
        }
    }

    fn build(mut self) -> Index<Self::ForBackend, T, S> {
        self.dict.finish();

        let postings: Vec<_> = self
            .postings_list
            .into_iter()
            .map(|list| {
                let postings = list
                    .into_iter()
                    .map(|(k, v)| (k, v.into_iter().collect()))
                    .collect();
                PP::from_map(postings).build()
            })
            .collect();

        let dict = self.dict.build();
        let storage = self.storage.build();

        Index::new(B::new(dict, postings, storage))
    }
}
