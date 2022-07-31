use index_framework::{
    backend::memory::{compr_postings, postings},
    traits::postings::{BuildPostings, IndexPostings},
};
use std::collections::HashMap;

#[test]
fn test_postings() {
    postings_test::<postings::Postings>();
    postings_test::<compr_postings::Postings>();
}

fn postings_test<P>()
where
    P: IndexPostings<List = Vec<u32>> + BuildPostings<Output = P, PostingList = Vec<u32>>,
{
    let len = 10000;
    let step = 111;

    let mut map: HashMap<u32, Vec<u32>> = HashMap::new();
    for i in (0..len).step_by(step) {
        map.entry(i).or_default().extend(i..i + step as u32);
    }

    let postings = P::from_map(map);

    for i in (0..len).step_by(step) {
        let posts = postings.get_posting(i);
        let exp_v: Vec<_> = (i as u32..i as u32 + step as u32).collect();
        assert_eq!(posts, exp_v);
    }
}
