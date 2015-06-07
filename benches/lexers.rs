#![feature(test)]

extern crate test;
extern crate luthor;

use test::Bencher;
use luthor::lexers::json::lex;

#[bench]
fn bench_json(b: &mut Bencher) {
    let data = include_str!("../test_data/data.json");
    b.iter(|| lex(data));
}
