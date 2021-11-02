#[derive(Debug)]
enum ScottOption<T> {
    None,
    Some(T),
}

pub fn enums() {
    let e = ScottOption::Some(5);
    let f = ScottOption::Some("hello");
    println!("e is {:?}; f is {:?}", e, f);

    let x: ScottOption<u32> = e;
    // type mismatch
    // let x: ScottOption<u32> = f;
    println!("x is {:?}", x);

    doTemp(Temperature::Fahrenheit(32));
    doTemp(Temperature::Celsius(0));
    doTemp(Temperature::Kelvin(272));
}

#[derive(Debug)]
pub enum Temperature {
    Fahrenheit(u32),
    Celsius(u32),
    Kelvin(u32)
}

fn doTemp(x: Temperature) {
    println!("temp {:?}", x);
}