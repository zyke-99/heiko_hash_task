use criterion::{black_box, criterion_group, criterion_main, Criterion};
use heiko_hash_task::{aggregator::{aggregate_hashes, aggregate_hashes_v2}, helpers::generate_number_of_random_hashes};

fn small_dataset_benchmarks(c: &mut Criterion) {
    let hashes = generate_number_of_random_hashes(100);

    let mut group = c.benchmark_group("Small Dataset");
    group.bench_function("Aggregate Hashes", |b| {
        b.iter(|| aggregate_hashes(black_box(&hashes)))
    });
    group.bench_function("Aggregate Hashes V2", |b| {
        b.iter(|| aggregate_hashes_v2(black_box(&hashes)))
    });
    group.finish();
}

fn large_dataset_benchmarks(c: &mut Criterion) {
    let hashes = generate_number_of_random_hashes(10_000);

    let mut group = c.benchmark_group("Large Dataset");
    group.bench_function("Aggregate Hashes", |b| {
        b.iter(|| aggregate_hashes(black_box(&hashes)))
    });
    group.bench_function("Aggregate Hashes V2", |b| {
        b.iter(|| aggregate_hashes_v2(black_box(&hashes)))
    });
    group.finish();
}

criterion_group!(benches, small_dataset_benchmarks, large_dataset_benchmarks);
criterion_main!(benches);