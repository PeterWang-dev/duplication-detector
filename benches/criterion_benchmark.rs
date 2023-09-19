use criterion::{black_box, criterion_group, criterion_main, Criterion};
use duplication_detector::{run, Config};
use std::time::Duration;

fn bench_add_something(c: &mut Criterion) {
    c.bench_function("add_something", |b| {
        b.iter(|| {
            run(black_box(Config::from(
                "example/inputs/orig.txt",
                "example/inputs/orig_0.8_add.txt",
                "output_0.8_add.txt",
            )))
        })
    });
}

fn bench_delete_something(c: &mut Criterion) {
    c.bench_function("delete_something", |b| {
        b.iter(|| {
            run(black_box(Config::from(
                "example/inputs/orig.txt",
                "example/inputs/orig_0.8_del.txt",
                "output_0.8_del.txt",
            )))
        })
    });
}

fn bench_change_distance_1(c: &mut Criterion) {
    c.bench_function("change_distance_1", |b| {
        b.iter(|| {
            run(black_box(Config::from(
                "example/inputs/orig.txt",
                "example/inputs/orig_0.8_dis_1.txt",
                "output_0.8_dis_1.txt",
            )))
        })
    });
}

fn bench_change_distance_10(c: &mut Criterion) {
    c.bench_function("change_distance_10", |b| {
        b.iter(|| {
            run(black_box(Config::from(
                "example/inputs/orig.txt",
                "example/inputs/orig_0.8_dis_10.txt",
                "output_0.8_dis_10.txt",
            )))
        })
    });
}

fn bench_change_distance_15(c: &mut Criterion) {
    c.bench_function("change_distance_15", |b| {
        b.iter(|| {
            run(black_box(Config::from(
                "example/inputs/orig.txt",
                "example/inputs/orig_0.8_dis_15.txt",
                "output_0.8_dis_15.txt",
            )))
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10).measurement_time(Duration::from_secs(30));
    targets = bench_add_something, bench_delete_something, bench_change_distance_1, bench_change_distance_10, bench_change_distance_15
}

criterion_main!(benches);
