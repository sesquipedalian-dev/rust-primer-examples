pub fn type_constraints() {
    println!("type constraint stuff");

    let h = Holder{t: String::from("42")};
    // following is NOT ok because we haven't implemented Intable for Holder<&str>
    // let h = Holder{t: "42"};
    let i = h.toInt();
    println!("Int translated!? {}", i);

    // don't compile because Holder does not impl trait Display
    //    Compiling first_rust v0.1.0 (/Users/scott.agnew/Documents/misc/first_rust)
    // error[E0277]: `Holder<String>` doesn't implement `std::fmt::Display`
    // --> src/type_constraints.rs:11:13
    //  |
    // 11 |     printer(h);
    //  |             ^ `Holder<String>` cannot be formatted with the default formatter
    //  |
    //  = help: the trait `std::fmt::Display` is not implemented for `Holder<String>`
    //  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    // note: required by a bound in `printer`
    // --> src/type_constraints.rs:15:15
    //  |
    // 15 | fn printer<T: std::fmt::Display>(t: T) {
    //  |               ^^^^^^^^^^^^^^^^^ required by this bound in `printer`
    // printer(h);
    printer(42); // ok
}

fn printer<T: std::fmt::Display>(t: T) {
    println!("{}", t);
}

trait Intable {
    fn toInt(&self) -> u32;
}

struct Holder<T> {
    t: T
}

impl Intable for Holder<String> {
    fn toInt(&self) -> u32 { 
        self.t.parse().unwrap()
    }
}

impl Intable for Holder<u32> {
    fn toInt(&self) -> u32 {
        self.t
    }
}