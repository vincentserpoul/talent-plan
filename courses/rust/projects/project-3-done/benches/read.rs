use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use kvs::KvStore;
use kvs::KvsEngine;
use tempfile::TempDir;

fn bench_read(c: &mut Criterion) {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    let mut kv_store = KvStore::open(temp_dir.path()).expect("unable to create kv store");
    let mut sled_store = sled::open(temp_dir.path()).expect("failed to open sled");

    kv_store
        .set("1".to_string(), "1".to_string())
        .expect("failed to set kv");
    sled_store
        .set("1".to_string(), "1".to_string())
        .expect("failed to set sled");

    let mut group = c.benchmark_group("KV read");

    group.bench_with_input(BenchmarkId::new("KV", 1), &1, |b, key| {
        b.iter(|| kv_store.get(key.to_string()))
    });
    group.bench_with_input(BenchmarkId::new("sled", 1), &1, |b, key| {
        b.iter(|| sled_store.get(key.to_string()))
    });

    group.finish();
}

criterion_group!(benches, bench_read);
criterion_main!(benches);
