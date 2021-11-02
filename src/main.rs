mod ownership;
mod enums;

use ownership::ownership;
use enums::enums;

fn main() {
    println!("Hello, world!");

    ownership();
    enums();
}
