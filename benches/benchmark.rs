use criterion::{criterion_group, criterion_main, Criterion};
use parser::dbs::parser::parse;

fn dbs() {
    let s = include_str!("../src/dbs/test1.txt");
    parse(vec![s.to_string()]);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("dbs", |b| b.iter(dbs));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
