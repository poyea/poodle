#![allow(unused)]
use poodle::state::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn state_benchmark(c: &mut Criterion) {
    let mut state = DayState::new("bench".to_string(), 6);
    c.bench_function("state guess wrong", |b| {
        b.iter(|| state.guess(black_box(&"chaos".to_string())))
    });

    c.bench_function("state guess correct", |b| {
        b.iter(|| state.guess(black_box(&"bench".to_string())))
    });

    c.bench_function("state guess wrong", |b| {
        b.iter(|| state.guess(black_box(&"wordy".to_string())))
    });
}

criterion_group!(benches, state_benchmark);
criterion_main!(benches);
