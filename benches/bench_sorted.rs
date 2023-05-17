#[macro_use]
extern crate criterion;

use criterion::{black_box, Bencher, BenchmarkId, Criterion, Throughput};
use sorted_rs::*;

fn do_ascending_bench(b: &mut Bencher, &size: &usize) {
    let nums = (0..size).into_iter().map(|u| u as i32).collect::<Vec<i32>>();
    b.iter(|| {
        is_sorted(&nums, Trend::Ascending);
        black_box(&nums);
    });
}

fn do_descending_bench(b: &mut Bencher, &size: &usize) {
    let mut nums = (0..size).into_iter().map(|u| u as i32).collect::<Vec<i32>>();
    nums.reverse();
    b.iter(|| {
        is_sorted(&nums, Trend::Descending);
        black_box(&nums);
    });
}

#[inline]
fn is_sorted_scalar<T: AsRef<[i32]>>(a: T, trend: Trend) -> bool {
    let a = a.as_ref();
    let compare = match trend {
        Trend::Ascending => |a: i32, b: i32| a > b,
        Trend::Descending => |a: i32, b: i32| a < b,
    };
    let len = a.as_ref().len();
    for i in 1..len {
        if compare(a[i - 1], a[i]) {
            return false;
        }
    }
    true
}

fn do_generic_ascending(b: &mut Bencher, &size: &usize) {
    let nums = (0..size).into_iter().map(|u| u as i32).collect::<Vec<i32>>();
    b.iter(|| {
        is_sorted_scalar(&nums, Trend::Ascending);
        black_box(&nums);
    });
}

fn do_generic_descending(b: &mut Bencher, &size: &usize) {
    let mut nums = (0..size).into_iter().map(|u| u as i32).collect::<Vec<i32>>();
    nums.reverse();
    b.iter(|| {
        is_sorted_scalar(&nums, Trend::Ascending);
        black_box(&nums);
    });
}

const SIZES: [usize; 5] = [4, 32, 64, 128, 256];
const LARGE_SIZES: [usize; 4] = [1024, 4096, 16384, 65536];

fn sorted_benchmarks(c: &mut Criterion, label: &str, sizes: &[usize]) {
    let mut group = c.benchmark_group(label);
    group
        .warm_up_time(std::time::Duration::from_millis(500))
        .measurement_time(std::time::Duration::from_secs(3));

    for size in sizes {
        group
            .throughput(Throughput::Bytes(*size as u64 * 4))
            .bench_with_input(
                BenchmarkId::new("ascending", size),
                size,
                do_ascending_bench,
            )
            .bench_with_input(
                BenchmarkId::new("descending", size),
                size,
                do_descending_bench,
            )
            .bench_with_input(
                BenchmarkId::new("generic ascending", size),
                size,
                do_generic_ascending,
            )
            .bench_with_input(
                BenchmarkId::new("generic descending", size),
                size,
                do_generic_descending,
            );
    }
    group.finish();
}

fn bench(c: &mut Criterion) {
    sorted_benchmarks(c, "sorted small", &SIZES[..]);
    sorted_benchmarks(c, "sorted large", &LARGE_SIZES[..]);
}

criterion_group!(benches, bench);
criterion_main!(benches);
