use std::collections::BTreeMap;
use std::iter::Iterator;
use std::ops::RangeBounds;
use std::vec::Vec;

use rand::{seq::SliceRandom, thread_rng, Rng};
use test::{black_box, Bencher};

macro_rules! map_insert_rand_bench {
    ($name: ident, $n: expr, $map: ident) => {
        #[bench]
        pub fn $name(b: &mut Bencher) {
            let n: usize = $n;
            let mut map = $map::new();
            // setup
            let mut rng = thread_rng();

            for _ in 0..n {
                let i = rng.gen::<usize>() % n;
                map.insert(i, i);
            }

            // measure
            b.iter(|| {
                let k = rng.gen::<usize>() % n;
                map.insert(k, k);
                map.remove(&k);
            });
            black_box(map);
        }
    };
}

macro_rules! map_insert_seq_bench {
    ($name: ident, $n: expr, $map: ident) => {
        #[bench]
        pub fn $name(b: &mut Bencher) {
            let mut map = $map::new();
            let n: usize = $n;
            // setup
            for i in 0..n {
                map.insert(i * 2, i * 2);
            }

            // measure
            let mut i = 1;
            b.iter(|| {
                map.insert(i, i);
                map.remove(&i);
                i = (i + 2) % n;
            });
            black_box(map);
        }
    };
}

macro_rules! map_find_rand_bench {
    ($name: ident, $n: expr, $map: ident) => {
        #[bench]
        pub fn $name(b: &mut Bencher) {
            let mut map = $map::new();
            let n: usize = $n;

            // setup
            let mut rng = thread_rng();
            let mut keys: Vec<_> = (0..n).map(|_| rng.gen::<usize>() % n).collect();

            for &k in &keys {
                map.insert(k, k);
            }

            keys.shuffle(&mut rng);

            // measure
            let mut i = 0;
            b.iter(|| {
                let t = map.get(&keys[i]);
                i = (i + 1) % n;
                black_box(t);
            })
        }
    };
}

macro_rules! map_find_seq_bench {
    ($name: ident, $n: expr, $map: ident) => {
        #[bench]
        pub fn $name(b: &mut Bencher) {
            let mut map = $map::new();
            let n: usize = $n;

            // setup
            for i in 0..n {
                map.insert(i, i);
            }

            // measure
            let mut i = 0;
            b.iter(|| {
                let x = map.get(&i);
                i = (i + 1) % n;
                black_box(x);
            })
        }
    };
}

map_insert_rand_bench! {insert_rand_100,    100,    BTreeMap}
map_insert_rand_bench! {insert_rand_10_000, 10_000, BTreeMap}

map_insert_seq_bench! {insert_seq_100,    100,    BTreeMap}
map_insert_seq_bench! {insert_seq_10_000, 10_000, BTreeMap}

map_find_rand_bench! {find_rand_100,    100,    BTreeMap}
map_find_rand_bench! {find_rand_10_000, 10_000, BTreeMap}

map_find_seq_bench! {find_seq_100,    100,    BTreeMap}
map_find_seq_bench! {find_seq_10_000, 10_000, BTreeMap}

fn bench_iteration(b: &mut Bencher, size: i32) {
    let mut map = BTreeMap::<i32, i32>::new();
    let mut rng = thread_rng();

    for _ in 0..size {
        map.insert(rng.gen(), rng.gen());
    }

    b.iter(|| {
        for entry in &map {
            black_box(entry);
        }
    });
}

#[bench]
pub fn iteration_20(b: &mut Bencher) {
    bench_iteration(b, 20);
}

#[bench]
pub fn iteration_1000(b: &mut Bencher) {
    bench_iteration(b, 1000);
}

#[bench]
pub fn iteration_100000(b: &mut Bencher) {
    bench_iteration(b, 100000);
}

fn bench_iteration_mut(b: &mut Bencher, size: i32) {
    let mut map = BTreeMap::<i32, i32>::new();
    let mut rng = thread_rng();

    for _ in 0..size {
        map.insert(rng.gen(), rng.gen());
    }

    b.iter(|| {
        for kv in map.iter_mut() {
            black_box(kv);
        }
    });
}

#[bench]
pub fn iteration_mut_20(b: &mut Bencher) {
    bench_iteration_mut(b, 20);
}

#[bench]
pub fn iteration_mut_1000(b: &mut Bencher) {
    bench_iteration_mut(b, 1000);
}

#[bench]
pub fn iteration_mut_100000(b: &mut Bencher) {
    bench_iteration_mut(b, 100000);
}

fn bench_first_and_last(b: &mut Bencher, size: i32) {
    let map: BTreeMap<_, _> = (0..size).map(|i| (i, i)).collect();
    b.iter(|| {
        for _ in 0..10 {
            black_box(map.first_key_value());
            black_box(map.last_key_value());
        }
    });
}

#[bench]
pub fn first_and_last_0(b: &mut Bencher) {
    bench_first_and_last(b, 0);
}

#[bench]
pub fn first_and_last_100(b: &mut Bencher) {
    bench_first_and_last(b, 100);
}

#[bench]
pub fn first_and_last_10k(b: &mut Bencher) {
    bench_first_and_last(b, 10_000);
}

const BENCH_RANGE_SIZE: i32 = 145;
const BENCH_RANGE_COUNT: i32 = BENCH_RANGE_SIZE * (BENCH_RANGE_SIZE - 1) / 2;

fn bench_range<F, R>(b: &mut Bencher, f: F)
where
    F: Fn(i32, i32) -> R,
    R: RangeBounds<i32>,
{
    let map: BTreeMap<_, _> = (0..BENCH_RANGE_SIZE).map(|i| (i, i)).collect();
    b.iter(|| {
        let mut c = 0;
        for i in 0..BENCH_RANGE_SIZE {
            for j in i + 1..BENCH_RANGE_SIZE {
                black_box(map.range(f(i, j)));
                c += 1;
            }
        }
        debug_assert_eq!(c, BENCH_RANGE_COUNT);
    });
}

#[bench]
pub fn range_included_excluded(b: &mut Bencher) {
    bench_range(b, |i, j| i..j);
}

#[bench]
pub fn range_included_included(b: &mut Bencher) {
    bench_range(b, |i, j| i..=j);
}

#[bench]
pub fn range_included_unbounded(b: &mut Bencher) {
    bench_range(b, |i, _| i..);
}

#[bench]
pub fn range_unbounded_unbounded(b: &mut Bencher) {
    bench_range(b, |_, _| ..);
}

fn bench_iter(b: &mut Bencher, repeats: i32, size: i32) {
    let map: BTreeMap<_, _> = (0..size).map(|i| (i, i)).collect();
    b.iter(|| {
        for _ in 0..repeats {
            black_box(map.iter());
        }
    });
}

/// Contrast range_unbounded_unbounded with `iter()`.
#[bench]
pub fn range_unbounded_vs_iter(b: &mut Bencher) {
    bench_iter(b, BENCH_RANGE_COUNT, BENCH_RANGE_SIZE);
}

#[bench]
pub fn iter_0(b: &mut Bencher) {
    bench_iter(b, 1_000, 0);
}

#[bench]
pub fn iter_1(b: &mut Bencher) {
    bench_iter(b, 1_000, 1);
}

#[bench]
pub fn iter_100(b: &mut Bencher) {
    bench_iter(b, 1_000, 100);
}

#[bench]
pub fn iter_10k(b: &mut Bencher) {
    bench_iter(b, 1_000, 10_000);
}

#[bench]
pub fn iter_1m(b: &mut Bencher) {
    bench_iter(b, 1_000, 1_000_000);
}
