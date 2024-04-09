struct Person {
    name: String,
    birth: u32,
    sex: char,
    height: f64,
    weight: f64,
}

fn main() {
    // mutableな構造体を作ることもできる
    // フィールド単位でmutableとimmutableを混在させることはできない
    let mut nao = Person {
        birth: 1945,
        height: 160.0,
        name: String::from("山内直"),
        sex: 'm',
        weight: 80.0,
    };

    nao.sex = 'f';
    println!("性別は {} です。", nao.sex);
}
