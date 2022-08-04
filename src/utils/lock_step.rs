use std::{cmp::Ordering, iter::Peekable};

pub(crate) struct LockStepIter<A, B, K>
where
    A: Iterator<Item = K>,
    B: Iterator<Item = K>,
{
    a: Peekable<A>,
    b: Peekable<B>,
}

impl<A, B, K> LockStepIter<A, B, K>
where
    A: Iterator<Item = K>,
    B: Iterator<Item = K>,
{
    #[inline]
    pub fn new(a: A, b: B) -> Self {
        Self {
            a: a.peekable(),
            b: b.peekable(),
        }
    }
}

impl<A, B, K> Iterator for LockStepIter<A, B, K>
where
    A: Iterator<Item = K>,
    B: Iterator<Item = K>,
    K: Ord,
{
    type Item = K;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match (self.a.peek(), self.b.peek()) {
                (Some(dim_a), Some(dim_b)) => match dim_a.cmp(dim_b) {
                    Ordering::Less => {
                        self.a.next()?;
                    }
                    Ordering::Greater => {
                        self.b.next()?;
                    }
                    Ordering::Equal => {
                        let k = unsafe { self.a.next().unwrap_unchecked() };
                        return Some(k);
                    }
                },
                _ => return None,
            }
        }
    }
}
