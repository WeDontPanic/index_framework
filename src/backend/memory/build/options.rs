use super::MemIndexBuilder;

/// Options for building indexes
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BuildOption {
    SortedPostings,
    UniquePostings,
}

/// API to modify postings before building
pub struct PostingsMod<B, T, S, DD, SS, PP> {
    filter_fn: Option<
        Box<dyn Fn(u32, u32, &mut Vec<u32>, &MemIndexBuilder<B, T, S, DD, SS, PP>) + 'static>,
    >,
}

impl<B, T, S, DD, SS, PP> PostingsMod<B, T, S, DD, SS, PP> {
    #[inline]
    pub fn new<F>(filter: F) -> Self
    where
        F: Fn(u32, u32, &mut Vec<u32>, &MemIndexBuilder<B, T, S, DD, SS, PP>) + 'static,
    {
        Self {
            filter_fn: Some(Box::new(filter)),
        }
    }

    #[inline]
    pub(crate) fn filter(
        &self,
        postings_id: u32,
        t_id: u32,
        vecs: &mut Vec<u32>,
        builder: &MemIndexBuilder<B, T, S, DD, SS, PP>,
    ) {
        if let Some(filter_fn) = self.filter_fn.as_ref() {
            (filter_fn)(postings_id, t_id, vecs, builder);
        }
    }
}

impl<B, T, S, DD, SS, PP> Default for PostingsMod<B, T, S, DD, SS, PP> {
    #[inline]
    fn default() -> Self {
        Self { filter_fn: None }
    }
}
