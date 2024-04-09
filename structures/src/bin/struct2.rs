struct Person {
    name: String,
    birth: u32,
    sex: char,
    height: f64,
    weight: f64,
}

fn main() {
    let nao = create_person(&"山内直".to_string(), 1960, 'f');
}

// フィールド名と関数の仮引数の名前が同じでもよい
fn create_person(name: &String, birth: u32, sex: char) -> Person {
    Person {
        name: name.to_string(),
        birth: birth,
        sex: sex,
        height: 0.0,
        weight: 0.0,
    }
}
