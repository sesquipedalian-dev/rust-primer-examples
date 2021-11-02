use std::collections::HashMap;

pub fn iterators() {
    println!("Iterator stuff");

    let mut m: HashMap<usize, usize> = HashMap::new();
    m.insert(1, 1);
    m.insert(2, 2);
    m.insert(3, 3);

    let v: Vec<(usize, usize)> = m
        .iter()
        .filter(|(k, v)| **v > 2)
        .map(|(k, v)| (*k, *v))
        .collect();

    for value in v {
        println!("{:?}", value);
    }
}