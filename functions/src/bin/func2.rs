fn main() {
    display_sum(10.5, 5);
}

fn display_sum(a: f64, b: i32) {
    let c = a + b as f64;
    println!("2つの引数の和は{}です。", c);
}
