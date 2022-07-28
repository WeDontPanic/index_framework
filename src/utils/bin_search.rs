use std::cmp::Ordering;

/// Generic bin search over any value
pub fn generic_binary_search<I, F, T>(
    over: I,
    mut size: usize,
    mut f: F,
) -> Result<(usize, T), usize>
where
    F: FnMut(&I, usize) -> (Ordering, T),
{
    let mut left = 0;
    let mut right = size;

    while left < right {
        let mid = left + size / 2;

        let (cmp, item) = f(&over, mid);

        if cmp == Ordering::Less {
            left = mid + 1;
        } else if cmp == Ordering::Greater {
            right = mid;
        } else {
            return Ok((mid, item));
        }

        size = right - left;
    }

    Err(left)
}
