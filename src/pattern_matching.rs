use crate::enums::Temperature;

pub fn pattern_matching() { 
    println!("match those patterns");

    let coord = (-1,2);
    println!("manhattan ({},{}) = ({})", coord.0, coord.1, manhattan(coord));

    let t1 = Temperature::Fahrenheit(451);
    let t2 = Temperature::Fahrenheit(0);
    println!("t1 is freezing? {}", is_freezing(t1));
    println!("t2 is freezing? {}", is_freezing(t2));

    // destructing / pattern matching in for loop
    let v = vec!('a', 'b', 'c');
    for (index, value) in v.iter().enumerate() {
        println!("i, v {} {}", index, value)
    }

    // destructuring in if let
    if let Some(v) = maybeAThing() {
        println!("Got some")
    } else {
        println!("got none")
    }
}

fn maybeAThing() -> Option<usize> {
    return None
}

// destructure in fn parameters
fn manhattan((x, y): (isize, isize)) -> usize {
    (x.abs() + y.abs()).try_into().unwrap()
}

// destructure in pattern match
fn is_freezing(t: Temperature) -> bool {
    match t{
        Temperature::Celsius(x) if x <= 0 => true,
        Temperature::Fahrenheit(x) if x <= 32 => true,
        Temperature::Kelvin(x) if x <= 272 => true,
        // if catch all is commented out it's a compiler error for not exhaustatively matching
        // error[E0004]: non-exhaustive patterns: `Fahrenheit(_)`, `Celsius(_)` and `Kelvin(_)` not covered --> src/pattern_matching.rs:39:11
        _ => false
    }
}
