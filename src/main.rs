mod ownership;
mod enums;
mod pattern_matching;
mod iterators;
mod type_constraints;

use ownership::ownership;
use enums::enums;
use pattern_matching::pattern_matching;
use iterators::iterators;
use type_constraints::type_constraints;

fn main() {
    println!("Hello, world!");

    ownership();
    enums();
    pattern_matching();
    iterators();
    type_constraints();
}
