pub mod options;

use crate::{
    traits::{
        backend::{Backend, NewBackend},
        build::IndexBuilder,
        deser::DeSer,
        dict_item::DictItem,
        dictionary::BuildIndexDictionary,
        postings::BuildPostings,
        storage::BuildIndexStorage,
    },
    Index,
};
use options::BuildOption;
use std::{collections::HashMap, hash::Hash, marker::PhantomData};

use self::options::PostingsMod;

/// Generic builder for memory indexes
pub struct MemIndexBuilder<B, T, S, DD, SS, PP> {
    pub dict: DD,
    pub storage: SS,
    pub postings_list: Vec<HashMap<u32, Vec<u32>>>,
    pub term_map: HashMap<T, u32>,
    options: Vec<BuildOption>,
    postings_mod: PostingsMod<B, T, S, DD, SS, PP>,
    s: PhantomData<S>,
    b: PhantomData<B>,
    p: PhantomData<PP>,
}

impl<B, T, S, DD, SS, PP> MemIndexBuilder<B, T, S, DD, SS, PP>
where
    B: Backend<T, S>,
    T: DictItem,
    S: DeSer,
    DD: BuildIndexDictionary<T, Output = B::Dict>,
    SS: BuildIndexStorage<S, Output = B::Storage>,
    PP: BuildPostings<Output = B::Postings, PostingList = Vec<u32>>,
{
    /// Create a new index builder
    #[inline]
    pub fn new() -> Self {
        Self::with_postings_len(1)
    }

    /// Create a new index builder with custom amount of postings
    #[inline]
    pub fn with_postings_len(postings_len: usize) -> Self {
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
            options: vec![],
            postings_mod: PostingsMod::default(),
            s: PhantomData,
            b: PhantomData,
            p: PhantomData,
        }
    }

    /// Sets the postings modifier
    #[inline]
    pub fn set_postings_mod(&mut self, pmod: PostingsMod<B, T, S, DD, SS, PP>) {
        self.postings_mod = pmod;
    }

    /// Adds a build-option to the builder
    #[inline]
    pub fn add_option(&mut self, option: BuildOption) {
        self.options.push(option)
    }

    /// Returns the dictionary value
    #[inline]
    pub fn dict(&self) -> &DD {
        &self.dict
    }

    /// Returns the storage value
    #[inline]
    pub fn storage(&self) -> &SS {
        &self.storage
    }

    /// Get a postings item by its ID
    #[inline]
    pub fn postings(&self, id: usize) -> Option<&HashMap<u32, Vec<u32>>> {
        self.postings_list.get(id)
    }

    /// Returns a map of terms to its IDs
    #[inline]
    pub fn term_map(&self) -> &HashMap<T, u32> {
        &self.term_map
    }

    /// Get a mutable postings item by its ID
    #[inline]
    pub fn postings_mut(&mut self, pos: usize) -> Option<&mut HashMap<u32, Vec<u32>>> {
        self.postings_list.get_mut(pos)
    }

    /// Returns the amount of posting-lists
    #[inline]
    pub fn postings_count(&self) -> usize {
        self.postings_list.len()
    }

    /// Returns `true` if bulider has the given option
    #[inline]
    fn has_option(&self, option: &BuildOption) -> bool {
        self.options.contains(option)
    }

    fn build_postings(&mut self) -> Vec<<B as Backend<T, S>>::Postings> {
        let postings_list = std::mem::take(&mut self.postings_list);
        let sort = self.has_option(&BuildOption::SortedPostings);

        let mut out = Vec::with_capacity(postings_list.len());
        for (postings_id, postings) in postings_list.into_iter().enumerate() {
            let mut tmp_map = HashMap::with_capacity(postings.len());

            for (t_id, mut ids) in postings {
                if sort {
                    ids.sort();
                }

                // Apply mod
                self.postings_mod
                    .filter(postings_id as u32, t_id, &mut ids, &self);

                tmp_map.insert(t_id, ids);
            }

            out.push(PP::from_map(tmp_map).build());
        }

        out
    }
}

impl<B, T, S, DD, SS, PP> IndexBuilder<T, S> for MemIndexBuilder<B, T, S, DD, SS, PP>
where
    B: Backend<T, S> + NewBackend<T, S>,
    T: DictItem + Hash + Clone,
    S: DeSer,
    DD: BuildIndexDictionary<T, Output = B::Dict>,
    SS: BuildIndexStorage<S, Output = B::Storage>,
    PP: BuildPostings<Output = B::Postings, PostingList = Vec<u32>>,
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
        let unique_postings = self.has_option(&BuildOption::UniquePostings);

        let postings = self
            .postings_mut(postings_id as usize)
            .expect("Invalid postings index");

        for term in terms {
            let entry = postings.entry(*term).or_default();

            if unique_postings && entry.contains(&item) {
                continue;
            }

            entry.push(item);
        }
    }

    fn build(mut self) -> Index<Self::ForBackend, T, S> {
        self.dict.finish();

        let postings = self.build_postings();
        let dict = self.dict.build();
        let storage = self.storage.build();

        Index::new(B::new(dict, postings, storage))
    }
}
