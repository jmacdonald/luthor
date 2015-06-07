#![feature(test)]

extern crate test;
extern crate luthor;

use test::Bencher;
use luthor::tokenizer::new;

#[bench]
fn bench_current_char(b: &mut Bencher) {
    let data = include_str!("../test_data/data.json");
    let tokenizer = new(data);
    b.iter(|| tokenizer.current_char());
}

#[bench]
fn bench_starts_with(b: &mut Bencher) {
    let data = include_str!("../test_data/data.json");
    let tokenizer = new(data);
    b.iter(|| tokenizer.starts_with("something"));
}
