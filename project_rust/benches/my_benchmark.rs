use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

#[path = "../src/red_black.rs"]
mod red_black;
use self::red_black::RBTree;
extern crate criterion;
extern crate rand;

fn rb_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Red_Black_Tree");

    for size in [10000, 40000, 70000, 100000, 130000].iter() {

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                // Creation of an empty tree
                let mut tree: RBTree<usize, usize> = RBTree::new();

                let search_size = size / 10;

                //Values with tree_size inserted into the tree
                for i in 0..=size {
                    let a = i as usize;
                    tree.insert(a, a).unwrap();
                }

                //Search for tree_size/10
                for i in 0..=search_size {
                    let a = i as usize;
                    tree.find_node(&a);
                }
            })
        });    
    }  
    group.finish();
}
criterion_group!(benches, rb_benchmark);
criterion_main!(benches);
