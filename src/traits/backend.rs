use super::{
    deser::DeSer, dict_item::DictItem, dictionary::IndexDictionary, postings::IndexPostings,
    storage::IndexStorage,
};
use std::{
    fs::File,
    io::{BufReader, Cursor, Read, Write},
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
    fn postings(&self) -> &Self::Postings;
    fn storage(&self) -> &Self::Storage;

    /// Returns `true` if the index doesn't contain index data
    #[inline]
    fn is_empty(&self) -> bool {
        self.dict().is_empty() || self.postings().is_empty() || self.storage().is_empty()
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
}

pub trait NewBackend<T, S>: Backend<T, S>
where
    T: DictItem,
    S: DeSer,
{
    fn new(dict: Self::Dict, postings: Self::Postings, storage: Self::Storage) -> Self;
}
