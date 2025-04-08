
// Fill the blank to make it work
fn main() {
    let x: f64 = 1_000.000_1; // ?
    let y: f32 = 0.12; // f32
    let z:f64 = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}