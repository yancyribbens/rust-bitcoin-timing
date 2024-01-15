use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_bitcoin_timing::bitcoin_sum;
use rust_bitcoin_timing::primitive_sum;
use bitcoin::Amount;
use core::str::FromStr;

pub fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("bitcoin_sum",
        |b| b.iter(|| {

            let mut start = Amount::ZERO;
            let one = Amount::from_str("1 sat").unwrap();
            let s = bitcoin_sum(black_box(start), black_box(one));
            assert_eq!(99999, s.to_sat())
        }));

	c.bench_function("primitive_sum",
        |b| b.iter(|| {
            let mut start = 0;
            let one = 1;
            let s = primitive_sum(black_box(start), black_box(one));
            assert_eq!(199999, s)
        }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
