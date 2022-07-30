use super::{
    deser::DeSer, dict_item::DictItem, dictionary::IndexDictionary, postings::IndexPostings,
    storage::IndexStorage,
};
use std::{
    fs::File,
    io::{BufReader, Cursor, Read, Write},
    marker::PhantomData,
    path::Path,
};

/// Defines how the index handles the data internally by having to specify type implementations
/// for all traits an index requires to work properly
pub trait Backend<T, S>
where
    T: DictItem,
    S: DeSer,
{
    type Dict: IndexDictionary<T>;
    type Postings: IndexPostings;
    type Storage: IndexStorage<S>;

    fn decode_from<R: Read>(reader: R) -> Option<Self>
    where
        Self: Sized;

    fn encode(&self) -> Vec<u8>;

    /// Encodes the index into a writer
    fn encode_to<W: Write>(&self, mut out: W) -> Result<(), std::io::Error> {
        let encoded = self.encode();
        Ok(out.write_all(&encoded)?)
    }

    fn dict(&self) -> &Self::Dict;

    /// Returns a postings list for the given ID
    fn postings(&self, id: u32) -> Option<&Self::Postings>;

    /// Returns the amount of posting maps
    fn posting_count(&self) -> usize;

    fn storage(&self) -> &Self::Storage;

    /// Returns `true` if the index doesn't contain index data
    #[inline]
    fn is_empty(&self) -> bool {
        self.dict().is_empty() || self.posting_count() == 0 || self.storage().is_empty()
    }

    /// Decodes an idnex from raw bytes
    fn decode(data: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        let r = Cursor::new(data);
        Self::decode_from(r)
    }

    /// Opens an index backend
    fn open<P: AsRef<Path>>(file: P) -> Option<Self>
    where
        Self: Sized,
    {
        let r = BufReader::new(File::open(file).ok()?);
        Self::decode_from(r)
    }

    /// Returns an iterator over all postings lists
    #[inline]
    fn postings_iter(&self) -> PostingsIterator<Self, T, S>
    where
        Self: Sized,
    {
        PostingsIterator::new(self)
    }
}

pub trait NewBackend<T, S>: Backend<T, S>
where
    T: DictItem,
    S: DeSer,
{
    fn new(dict: Self::Dict, postings: Vec<Self::Postings>, storage: Self::Storage) -> Self;
}

pub struct PostingsIterator<'a, B, T, S> {
    backend: &'a B,
    pos: u32,
    p1: PhantomData<T>,
    p2: PhantomData<S>,
}

impl<'a, B, T, S> PostingsIterator<'a, B, T, S>
where
    B: Backend<T, S>,
    T: DictItem,
    S: DeSer,
{
    #[inline]
    fn new(backend: &'a B) -> Self {
        Self {
            backend,
            pos: 0,
            p1: PhantomData,
            p2: PhantomData,
        }
    }
}

impl<'a, B, T, S> Iterator for PostingsIterator<'a, B, T, S>
where
    B: Backend<T, S>,
    <B as Backend<T, S>>::Postings: 'a,
    T: DictItem,
    S: DeSer,
{
    type Item = &'a B::Postings;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let p = self.backend.postings(self.pos)?;
        self.pos += 1;
        Some(p)
    }
}
