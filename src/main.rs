use functools::cache;
use std::collections::HashMap;

mod toolbox;

use toolbox::point::Point;

fn main() {
    println!("{:?}", fibonacci(50));
    println!("{:?}", Point::new(3, 4));
}

#[cache]
fn fibonacci(n: usize) -> usize {
    match n {
        0 | 1 => n,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
