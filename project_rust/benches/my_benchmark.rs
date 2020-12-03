use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

#[path = "../src/red_black.rs"]
mod lib;
use lib::*;
extern crate criterion;
extern crate rand;

 pub fn criterion_benchmark(c: &mut Criterion) {
 	let number: [u32; 5] = [1000, 2000, 3000, 4000, 5000];

 	for number in number.iter() {

        c.bench_function("Red Black Tree", |b| {
            b.iter(|| {
                // Creation of an empty tree
                let mut tree: RBTree<usize, usize> = RBTree::new();

                let search_size = *&number / 10;

                //Values with tree_size inserted into the tree
                for i in 1..=*number {
                    let mut a = i as usize;
                    tree.insert(a, a).unwrap();
                }

                //Search for tree_size/10
                for i in 1..=search_size {
                    //tree.search_function(&i);
                }
            })
        });
 }
}
criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);           
    targets = criterion_benchmark
}
criterion_main!(benches);
