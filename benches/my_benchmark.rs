use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_bitcoin_timing::bitcoin_sum;
use rust_bitcoin_timing::primitive_sum;

pub fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("bitcoin_sum",
        |b| b.iter(|| {
            let s = black_box(bitcoin_sum());
            assert_eq!(99999, s.to_sat())
        }));

	c.bench_function("primitive_sum",
        |b| b.iter(|| {
            let s = black_box(primitive_sum());
            assert_eq!(99999, s)
        }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
