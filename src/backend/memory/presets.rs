use super::{
    dict::{self, fixed_len::FixDict},
    postings, storage, MemBackend,
};

/// N-gram Index
pub type NGIndex<const N: usize, T, S> =
    MemBackend<T, S, FixDict<N>, storage::default::Storage<S>, postings::compressed::Postings>;

// Simple Index
pub type Simple<T, S> = MemBackend<
    T,
    S,
    dict::default::Dictionary<T>,
    storage::default::Storage<S>,
    postings::default::Postings,
>;

// Simple compressed Index
pub type SimpleCompressed<T, S> = MemBackend<
    T,
    S,
    dict::default::Dictionary<T>,
    storage::default::Storage<S>,
    postings::compressed::Postings,
>;

// Compressed u32-index
pub type CompressedU32<T> = MemBackend<
    T,
    u32,
    dict::default::Dictionary<T>,
    storage::c_u32::U32Storage,
    postings::compressed::Postings,
>;
