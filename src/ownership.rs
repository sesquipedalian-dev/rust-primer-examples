
pub fn ownership() {
    let s1 = String::from("hello");

    // this isn't OK because the change function borrows mutably
    // change(&s1);

    let mut s1 = String::from("hello");
    change(&mut s1);

    println!("Len ({}) is ({})", s1, strlen(&s1));

    // creating a block here allows two simultaneously mutable references to S2
    // because r1 goes out of scope
    let mut s2 = String::from("goodbye");
    {
        let r1 = &mut s2;
    }
    let r2 = &mut s2;
    println!("r is {}", r2);
}

fn strlen(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("hello")
}

// this is not OK because push_str requires a mut reference
// fn change(s: &String) {
//     s.push_str("hello")
// }

// this is not OK because s1 was created in the scope of this function, but if we return a
// reference the object would already be deleted
//   --> src/ownership.rs:38:5
// |
// 38 |     &s
//    |     ^^ returns a reference to data owned by the current function
// fn dangle() -> &'static String {
//     let s = String::from("1");
//     &s
// }