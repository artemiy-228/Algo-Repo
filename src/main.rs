use rand::Rng;

mod sort;

use crate::sort::*;

fn main() {
    let mut v1: Vec<i32> = Vec::new();

    let mut generator = rand::thread_rng();
    let v1_size = 10;

    for _i in 0..=v1_size {
        v1.push(generator.gen_range(-50..=50));
    }
    println!("{:?}", v1);

    let sorted_v1 = merge_sort(&mut v1);

    println!("{:?}", sorted_v1);
}
