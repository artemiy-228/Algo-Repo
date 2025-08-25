use rand::Rng;

mod bubble_sort;

use crate::bubble_sort::*;

fn main() {
    let mut v1: Vec<i32> = Vec::new();

    let mut generator = rand::thread_rng();
    let v1_size = 10;

    for _i in 0..=v1_size {
        v1.push(generator.gen_range(-50..=50));
    }
    println!("{:?}", v1);

    bubble_sort(&mut v1);

    println!("{:?}", v1);
}
