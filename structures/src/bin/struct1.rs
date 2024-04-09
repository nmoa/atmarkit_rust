struct Person {
    name: String,
    birth: u32,
    sex: char,
    height: f64,
    weight: f64,
}

fn main() {
    // 初期化の順序が構造体の宣言と同じでなくてもよい
    let nao = Person {
        birth: 1945,
        height: 160.0,
        name: String::from("山内直"),
        sex: 'm',
        weight: 80.0,
    };
}
