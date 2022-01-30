extern crate test;
use crate::r#impl::*;
use rand::distributions::Alphanumeric;
use rand::prelude::*;
use rand_pcg::Pcg64;
use std::iter::repeat;
use std::ops::Range;
use test::Bencher;

fn strs(count: usize, ref len: Range<usize>) -> Vec<String> {
    let mut rng = Pcg64::seed_from_u64(count as u64);
    (0..count)
        .map(|_| {
            (0..rng.gen_range(len.clone()))
                .map(|_| -> char { rng.sample(&Alphanumeric).into() })
                .collect::<String>()
        })
        .collect()
}

fn front(inp: Vec<String>, ref len: Range<usize>) -> Vec<String> {
    let mut rng = Pcg64::seed_from_u64(42);
    inp.into_iter()
        .map(|s| " ".repeat(rng.gen_range(len.clone())) + &s)
        .collect()
}

fn back(inp: Vec<String>, ref len: Range<usize>) -> Vec<String> {
    let mut rng = Pcg64::seed_from_u64(42);
    inp.into_iter()
        .map(|s| s + &" ".repeat(rng.gen_range(len.clone())))
        .collect()
}

fn shortstr() -> Vec<String> {
    strs(100, 2..5)
}
fn longstr() -> Vec<String> {
    strs(100, 60..120)
}

fn nosp(inp: Vec<String>) -> Vec<String> {
    inp
}
fn shortsp(inp: Vec<String>) -> Vec<String> {
    front(back(inp, 1..3), 1..3)
}
fn longsp(inp: Vec<String>) -> Vec<String> {
    front(back(inp, 30..60), 30..60)
}

macro_rules! bench {
    ($fn:ident, $core:ident, $space:ident) => {
        paste::paste! {
            #[bench]
            fn [<$space _ $core _ $fn>](b: &mut Bencher) {
                let data = $space($core());
                let mut data = repeat(data.iter()).flatten();
                b.iter(|| $fn(test::black_box(data.next().unwrap().to_owned())));
            }
        }
    };
}

bench!(r#unsafe, shortstr, nosp);
bench!(r#unsafe, shortstr, shortsp);
bench!(r#unsafe, shortstr, longsp);
bench!(r#unsafe, longstr, nosp);
bench!(r#unsafe, longstr, shortsp);
bench!(r#unsafe, longstr, longsp);

bench!(middle, shortstr, nosp);
bench!(middle, shortstr, shortsp);
bench!(middle, shortstr, longsp);
bench!(middle, longstr, nosp);
bench!(middle, longstr, shortsp);
bench!(middle, longstr, longsp);

bench!(safe, shortstr, nosp);
bench!(safe, shortstr, shortsp);
bench!(safe, shortstr, longsp);
bench!(safe, longstr, nosp);
bench!(safe, longstr, shortsp);
bench!(safe, longstr, longsp);

bench!(drain, shortstr, nosp);
bench!(drain, shortstr, shortsp);
bench!(drain, shortstr, longsp);
bench!(drain, longstr, nosp);
bench!(drain, longstr, shortsp);
bench!(drain, longstr, longsp);

bench!(naive_opt, shortstr, nosp);
bench!(naive_opt, shortstr, shortsp);
bench!(naive_opt, shortstr, longsp);
bench!(naive_opt, longstr, nosp);
bench!(naive_opt, longstr, shortsp);
bench!(naive_opt, longstr, longsp);
