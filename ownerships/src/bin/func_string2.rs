fn main() {
    let mut s = String::from("Hello, world!");
    s = function_move(s); // 返り値として文字列の所有権を受け取っているので、この後もsを使うことができる
    println!("sは「{}」です。", s);
}

fn function_move(m: String) -> String {
    println!("function_move: 引数mの値は「{}」です。", m);
    m // 文字列を返すことで呼び出し元に対して所有権を移動することができる
}
