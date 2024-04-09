struct Person {
    name: String,
    birth: u32,
    sex: char,
    height: f64,
    weight: f64,
}

// impl文を使って構造体にメソッドを追加する
impl Person {
    fn bmi(&self) -> f64 {
        self.weight / ((self.height / 100.0) * (self.height / 100.0))
    }
}

fn main() {
    let nao = Person {
        name: "山内直".to_string(),
        birth: 1945,
        sex: 'm',
        height: 160.0,
        weight: height / 2,
    };

    println!("{}さんの体重は{}です。", nao.name, nao.weight);
    println!("{}さんのBMIは{}です。", nao.name, nao.bmi());
}
