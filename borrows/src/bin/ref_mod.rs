fn main() {
    let mut s = String::from("Hello");

    println!("変更前の文字列は「{}」です。", s);
    change_string(&mut s); // &mut s で可変参照を渡すことができる
    println!("変更された文字列は「{}」です。", s);

    // 以下のように変更可能な参照を複数作成することはできない
    // let r1 = &mut s;
    // let r2 = &mut s; // ここでコンパイルエラーになる
    // println!("{}, {}", r1, r2);
}

fn change_string(s: &mut String) {
    s.push_str(", Rust!!");
}
