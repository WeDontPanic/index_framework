use super::deser::DeSer;

/// A dictionary item
pub trait DictItem: DeSer + Eq + Ord {}

impl<T: DeSer + Eq + Ord> DictItem for T {}
