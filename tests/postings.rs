use index_framework::{
    backend::memory::postings::{compressed, default},
    traits::postings::{BuildPostings, IndexPostings},
};
use rand::{thread_rng, Rng};
use std::collections::{HashMap, HashSet};

#[test]
fn test_postings() {
    postings_test::<default::Postings>();
    postings_test::<compressed::Postings>();
}

fn postings_test<P>()
where
    P: IndexPostings + BuildPostings<Output = P, PostingList = Vec<u32>>,
{
    let mut map: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut rand = thread_rng();
    let mut did: HashSet<u32> = HashSet::new();

    let count = 100000;
    for _ in 0..count {
        let key = rand.gen_range(0..count);
        if did.contains(&key) {
            continue;
        }
        did.insert(key);

        let size = rand.gen_range(0..100);
        let data = (0..size).map(|_| rand.gen_range(1..1000000));
        map.entry(key).or_default().extend(data);
    }

    let postings = P::from_map(map.clone());

    for (k, v) in map {
        let posts = postings.get_posting(k);
        assert_eq!(posts, v);
    }
}

/* fn postings_test2<P>()
where
    P: IndexPostings + BuildPostings<Output = P, PostingList = Vec<u32>>,
{
    let file = BufReader::new(File::open("/home/jojii/tmp/out").unwrap());
    let map: HashMap<u32, Vec<u32>> = bincode::deserialize_from(file).unwrap();

    let postings = P::from_map(map.clone());

    for (k, v) in map {
        let posts = postings.get_posting(k);
        assert_eq!(posts, v);
    }
} */
