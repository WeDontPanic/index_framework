use std::marker::PhantomData;

/// Storage containing all 'result' items in the index
pub trait IndexStorage<I> {
    fn get_item(&self, id: u32) -> Option<I>;
    fn has_item(&self, id: u32) -> bool;

    /// Returns the amount of items in the storage
    fn len(&self) -> usize;

    /// Returns an iterator over all terms in the dictionary
    #[inline]
    fn iter(&self) -> StorageIter<Self, I>
    where
        Self: Sized,
    {
        StorageIter::new(self)
    }

    /// Returns `true` if there is no item in the storage
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait BuildIndexStorage<T> {
    type Output;

    fn new() -> Self;

    fn insert(&mut self, item: T) -> u32;

    fn build(self) -> Self::Output;
}

pub struct StorageIter<'a, D: IndexStorage<I>, I> {
    inner: &'a D,
    p: PhantomData<I>,
    pos: usize,
}

impl<'a, D, I> StorageIter<'a, D, I>
where
    D: IndexStorage<I>,
{
    #[inline]
    pub(crate) fn new(inner: &'a D) -> Self {
        Self {
            inner,
            p: PhantomData,
            pos: 0,
        }
    }
}

impl<'a, D, I> Iterator for StorageIter<'a, D, I>
where
    D: IndexStorage<I>,
{
    type Item = I;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inner.get_item(self.pos as u32)?;
        self.pos += 1;
        Some(item)
    }
}
