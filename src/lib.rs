pub mod backend;
pub mod error;
pub mod retrieve;
pub mod traits;
pub mod utils;

use crate::traits::backend::Backend;
use retrieve::Retrieve;
use serde::{Deserialize, Serialize};
use std::{marker::PhantomData, ops::Deref, path::Path};
use traits::{deser::DeSer, dict_item::DictItem, postings::IndexPostings};

#[derive(Serialize, Deserialize)]
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

impl<B, T, S> Index<B, T, S>
where
    B: Backend<T, S>,
    T: DictItem,
    S: DeSer,
    <<B as Backend<T, S>>::Postings as IndexPostings>::List: IntoIterator<Item = u32>,
{
    #[inline]
    pub fn retrieve(&self) -> Retrieve<'_, B, T, S> {
        Retrieve::new(self)
    }
}

impl<B, T, S> Deref for Index<B, T, S> {
    type Target = B;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.b
    }
}
