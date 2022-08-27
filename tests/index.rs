use index_framework::{
    backend::memory::{
        build::MemIndexBuilder,
        dict::default::Dictionary,
        postings::{compressed, default},
        storage::default::Storage,
        MemBackend,
    },
    retrieve::retriever::default::DefaultRetrieve,
    traits::{
        backend::Backend,
        build::IndexBuilder,
        dictionary::{BuildIndexDictionary, IndexDictionary},
        postings::BuildPostings,
        storage::{BuildIndexStorage, IndexStorage},
    },
    traits::{deser::DeSer, postings::IndexPostings},
    Index,
};
use std::collections::HashMap;

/// Dummy "documents" that will get indexed
const DOCS: &[&str] = &[
    "日本語 で 書いた テキスト です",
    "this is some text to tindex",
    "some other text",
    "lol text text",
    "impl<B, T, S, DD, SS, PP> MemIndexBuilder<B, T, S, DD, SS, PP>
where
    B: Backend<T, S>,
    T: DictItem,
    S: DeSer,
    DD: BuildIndexDictionary<T, Output = B::Dict>,
    SS: BuildIndexStorage<S, Output = B::Storage>,
    PP: BuildPostings<Output = B::Postings, PostingList = <B::Postings as IndexPostings>::List>, 
source
pub fn new(postings_len: usize) -> Self
source
pub fn dict(&self) -> &DD
source
pub fn storage(&self) -> &SS
source
pub fn postings(&self, id: usize) -> Option<&HashMap<u32, Vec<u32>>>
source
pub fn term_map(&self) -> &HashMap<T, u32>
source
pub fn postings_mut(
    &mut self,
    pos: usize
) -> Option<&mut HashMap<u32, Vec<u32>>>",
];

struct TestSet<D, S, P>
where
    D: IndexDictionary<String>,
    S: IndexStorage<u32>,
    P: IndexPostings,
{
    index: Index<MemBackend<String, u32, D, S, P>, String, u32>,

    // Maps Term to its ID
    term_id_map: HashMap<String, u32>,

    // Maps document ID to its position in `DOCS`
    item_id_map: HashMap<u32, u32>,
}

fn new_testset<D, P, S, BD, BP, BS>() -> TestSet<D, S, P>
where
    D: IndexDictionary<String> + DeSer,
    P: IndexPostings + DeSer,
    S: IndexStorage<u32> + DeSer,
    BD: BuildIndexDictionary<String, Output = D>,
    BP: BuildPostings<Output = P, PostingList = Vec<u32>>,
    BS: BuildIndexStorage<u32, Output = S>,
{
    let mut builder: MemIndexBuilder<MemBackend<String, u32, D, S, P>, String, u32, BD, BS, BP> =
        MemIndexBuilder::new();

    // Maps Term to its ID
    let mut term_id_map: HashMap<String, u32> = HashMap::new();

    // Maps document ID to its position in `DOCS`
    let mut item_id_map: HashMap<u32, u32> = HashMap::new();

    for (pos, doc) in DOCS.iter().enumerate() {
        let term_ids: Vec<_> = doc
            .split(' ')
            .map(|i| {
                let id = builder.insert_term(i.to_string()).unwrap_or_else(|v| v);
                term_id_map.insert(i.to_string(), id);
                id
            })
            .collect();
        let item_id = builder.index_new(0, pos as u32, &term_ids);
        item_id_map.insert(item_id, pos as u32);
    }

    let index = builder.build();
    TestSet {
        index,
        term_id_map,
        item_id_map,
    }
}

impl<D, S, P> TestSet<D, S, P>
where
    D: IndexDictionary<String> + DeSer,
    S: IndexStorage<u32> + DeSer,
    P: IndexPostings + DeSer,
{
    fn test(&self) {
        self.test_index();
        self.test_retrieve_iter();
    }

    fn test_index(&self) {
        let index = &self.index;

        for (ex_term, ex_id) in self.term_id_map.iter() {
            let id = index.dict().get_id(ex_term);
            assert_eq!(id, Some(*ex_id));
            let term = index.dict().get_term(id.unwrap());
            assert_eq!(term.as_ref(), Some(ex_term));
        }

        for (item_id, doc_pos) in self.item_id_map.iter() {
            let storage = index.storage();
            let doc = storage.get_item(*item_id).unwrap();
            let doc = &DOCS[doc as usize];
            let ex_doc = &DOCS[*doc_pos as usize];
            assert_eq!(doc, ex_doc);
        }
    }

    fn test_retrieve_iter(&self) {
        let res = self
            .index
            .retrieve()
            .by_terms(["text"])
            .unique()
            .get_all::<DefaultRetrieve<_, _, _>>();
        assert_eq!(res, vec![3, 2, 1]);
    }
}

#[test]
fn test_builder() {
    new_testset::<_, _, _, Dictionary<_>, compressed::Postings, Storage<_>>().test();
    new_testset::<_, _, _, Dictionary<_>, default::Postings, Storage<_>>().test();
}
