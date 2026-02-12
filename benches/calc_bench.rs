use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hd_cli::calc::build_chart;

fn bench_build_chart_basic(c: &mut Criterion) {
    c.bench_function("build_chart_basic", |b| {
        b.iter(|| {
            build_chart(
                black_box(1990),
                black_box(5),
                black_box(15),
                black_box(14),
                black_box(30),
                black_box(3.0),
                black_box(false), // short mode
                black_box("ru"),
            )
        })
    });
}

fn bench_build_chart_full(c: &mut Criterion) {
    c.bench_function("build_chart_full", |b| {
        b.iter(|| {
            build_chart(
                black_box(1990),
                black_box(5),
                black_box(15),
                black_box(14),
                black_box(30),
                black_box(3.0),
                black_box(true), // full descriptions
                black_box("ru"),
            )
        })
    });
}

criterion_group!(benches, bench_build_chart_basic, bench_build_chart_full);
criterion_main!(benches);
