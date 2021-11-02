mod ownership;
mod enums;
mod pattern_matching;
mod iterators;

use ownership::ownership;
use enums::enums;
use pattern_matching::pattern_matching;
use iterators::iterators;

fn main() {
    println!("Hello, world!");

    ownership();
    enums();
    pattern_matching();
    iterators();
}
