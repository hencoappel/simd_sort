#[macro_use]
extern crate criterion;

use criterion::BenchmarkId;
use criterion::Criterion;
use rand::{Rng, thread_rng};

use simd_sort::quick_sort;

fn rust_sort_benchmark(c: &mut Criterion) {
    bench_sort_func(c, "Rust sort", |v| v.sort());
}

fn rust_unstable_sort_benchmark(c: &mut Criterion) {
    bench_sort_func(c, "Rust sort unstable", |v| v.sort_unstable());
}

fn quicksort_benchmark(c: &mut Criterion) {
    bench_sort_func(c, "quicksort", quick_sort);
}

fn bench_sort_func<F>(c: &mut Criterion, group_name: &str, mut f: F) where F: FnMut(&mut [i32]) {
    let mut group = c.benchmark_group(group_name);
    for n in (1..4).map(|i| 10_usize.pow(i)) {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| {
                f(&mut random_array(n));
            });
        });
    }
}

fn random_array(n: usize) -> Vec<i32>{
    thread_rng().gen_iter::<i32>().take(n).collect()
}

criterion_group!(
    benches,
    rust_sort_benchmark,
    rust_unstable_sort_benchmark,
    quicksort_benchmark
);
criterion_main!(benches);
