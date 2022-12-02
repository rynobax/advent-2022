use std::collections::BinaryHeap;
use std::fs;

fn main() {
    let mut heap = BinaryHeap::from([1, 3]);
    let contents = fs::read_to_string("/mnt/f/advent-2022/input.txt").expect("Missing file");
    contents.split("\n\n").for_each(|backpack| {
        let calories = backpack.split("\n").fold(0, |acc, n_str| {
            return acc + n_str.parse::<i32>().expect("A number");
        });
        heap.push(calories);
    });
    let mut sum = 0;
    for _ in 0..3 {
        let val = heap.pop().expect("A value");
        println!("{val}");
        sum += val;
    }
    println!("Sum: {sum}");
}
