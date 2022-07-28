use serde::{de::DeserializeOwned, Serialize};
use std::io::Read;

/// Generalizing & Simplifying serailizing and deserializing
pub trait DeSer: Serialize + DeserializeOwned {
    #[inline(always)]
    fn encode_vec(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(10);
        bincode::serialize_into(&mut out, self).expect("Encoding failed");
        out
    }

    #[inline(always)]
    fn decode_vec(data: &[u8]) -> Option<Self> {
        bincode::deserialize(data).ok()
    }

    #[inline(always)]
    fn decode<R: Read>(reader: R) -> Option<Self> {
        bincode::deserialize_from(reader).ok()
    }
}

impl<T: Serialize + DeserializeOwned> DeSer for T {}
