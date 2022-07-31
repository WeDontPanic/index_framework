pub mod backend;
pub mod error;
pub mod traits;
pub mod utils;

use crate::traits::{backend::Backend, build::IndexBuilder};
use backend::memory::{
    backend::MemoryBackend, builder::MemIndexBuilder, dict::default::Dictionary,
    postings::compressed::Postings, storage::default::Storage,
};
use std::{io::Write, marker::PhantomData, ops::Deref, path::Path};
use traits::{deser::DeSer, dict_item::DictItem};

pub struct Index<B, T, S> {
    b: B,
    p1: PhantomData<T>,
    p2: PhantomData<S>,
}

impl<B, T, S> Index<B, T, S>
where
    B: Backend<T, S>,
    T: DictItem,
    S: DeSer,
{
    #[inline]
    pub(crate) fn new(b: B) -> Self {
        Self {
            b,
            p1: PhantomData,
            p2: PhantomData,
        }
    }

    #[inline]
    pub fn open<P: AsRef<Path>>(&self, path: P) -> Option<Self> {
        Some(Self::new(B::open(path)?))
    }
}

impl<B, T, S> Deref for Index<B, T, S> {
    type Target = B;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.b
    }
}

fn main() {
    /*
    let mut builder: MemIndexBuilder<
        MemoryBackend<_, _>,
        _,
        _,
        Dictionary<_>,
        Storage<_>,
        Postings,
    > = MemIndexBuilder::new(1);

    let res = resources::load_raw("./storage_data").unwrap();
    let wlen = res.words().count();
    let mut original_len = 0;

    for (pos, word) in res.words().iter().enumerate() {
        let terms = word
            .sense_gloss_iter()
            .map(|i| {
                i.1.gloss
                    .as_str()
                    .split(' ')
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();

        original_len += terms.len();

        let mut tids = vec![];

        for term in terms {
            let e = match builder.insert_term(term) {
                Ok(e) => e,
                Err(e) => e,
            };
            tids.push(e);
        }

        builder.index_item(1, word.sequence, &tids);

        if pos % 100 == 0 {
            print!("\r{pos}/{wlen}");
            std::io::stdout().flush().unwrap();
        }
    }
    println!("");

    println!("oringial len: {original_len}");

    let index = builder.build();
    println!("Total index size: {}", index.encode().len());

    let storage_size = index.storage().encode_vec().len();
    let posting_size = index.postings(0).unwrap().encode_vec().len();
    let dict_size = index.dict().encode_vec().len();
    println!("Storage size: {storage_size}");
    println!("Postings size: {posting_size}");
    println!("Dict size: {dict_size}");
    */
}
