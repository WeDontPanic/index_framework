use criterion::{black_box, criterion_group, criterion_main, Criterion};
use index_framework::{
    backend::memory::{
        backend::MemoryBackend, builder::MemIndexBuilder, compr_postings::Postings,
        dict::Dictionary, storage::Storage,
    },
    traits::{backend::Backend, build::IndexBuilder, dictionary::IndexDictionary},
    Index,
};

fn get_simple_index() -> Index<MemoryBackend<String, u32>, String, u32> {
    let mut builder: MemIndexBuilder<
        MemoryBackend<String, u32>,
        String,
        u32,
        Dictionary<String>,
        Storage<u32>,
        Postings,
    > = MemIndexBuilder::new(1);

    let res = resources::load_raw("./storage_data").unwrap();
    for word in res.words().iter() {
        let reading = word.get_reading().reading.clone();
        let e = builder.insert_term(reading).unwrap_or_else(|v| v);
        builder.index_item(0, word.sequence, &[e]);
    }
    builder.build()
}

fn index_item_decode(c: &mut Criterion) {
    let index = get_simple_index();

    c.bench_function("get term", |b| {
        let terms: Vec<_> = ["今日", "煩い", "運動", "嬉しい", "穴"]
            .into_iter()
            .map(|i| i.to_string())
            .collect();

        b.iter(|| {
            for term in &terms {
                let _e = index.dict().get_id(black_box(term)).unwrap();
            }
        });
    });
}

criterion_group!(benches, index_item_decode);
criterion_main!(benches);
