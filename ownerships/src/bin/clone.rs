fn main() {
    let s1 = String::from("Hello, Rust!");
    let s2 = s1.clone(); // 文字列を複製する。実体のメモリをヒープに確保して文字列をコピーするので、少し負荷が高い。
    println!("s1は{}です。", s1);
    println!("s2は{}です。", s2);
}
