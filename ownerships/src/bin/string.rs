fn main() {
    // String型の変数を生成する。文字列の実体はヒープに確保され、スタックにはポインタ・長さ・容量のデータが格納される
    let s1 = String::from("Hello, Rust!");

    // let s1 = "Hello, Rust!".to_string(); // この書き方でもOK.
    // let s1 = "Hello, Rust!"; // こう書くと文字列リテラルを参照する型になる

    let s2 = s1; // 所有権をs2に移動する
                 // println!("s1は{}です。", s1); // s1は所有権を失っているので、参照することができない。コンパイルエラーになる
    println!("s2は{}です。", s2);
}
