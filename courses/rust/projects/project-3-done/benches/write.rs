use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use kvs::KvStore;
use kvs::KvsEngine;
use tempfile::TempDir;

fn bench_write(c: &mut Criterion) {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    let mut kv_store = KvStore::open(temp_dir.path()).expect("unable to create kv store");
    let mut sled_store = sled::open(temp_dir.path()).expect("failed to open sled");

    let mut group = c.benchmark_group("KV write");

    group.bench_with_input(BenchmarkId::new("KV", 1), &1, |b, key| {
        b.iter(|| kv_store.set(key.to_string(), "1".to_string()))
    });
    group.bench_with_input(BenchmarkId::new("sled", 1), &1, |b, key| {
        b.iter(|| sled_store.set(key.to_string(), "1".to_string()))
    });

    group.finish();
}

criterion_group!(benches, bench_write);
criterion_main!(benches);
