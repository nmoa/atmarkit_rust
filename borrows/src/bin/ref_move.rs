// 注意：このソースはコンパイルエラーになります。
fn main() {
    let s1 = String::from("hello");
    let r = &s1;
    println!("s1は「{}」です。", s1);
    let s2 = s1; // 所有権を借用しているときに所有権の移動や破棄をしようとするとエラーになる
    println!("s2は「{}」です。", s2);
    println!("rは「{}」です。", r);
}
