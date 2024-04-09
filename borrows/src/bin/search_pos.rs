fn main() {
    let s1 = String::from("Hello, world!!");
    let c = 'w';

    let (s2, pos) = search_position(s1, c);
    println!("文字'{}'の「{}」中の位置は{}文字目です。", c, s2, pos);
}

fn search_position(s: String, c: char) -> (String, usize) {
    let pos = s.find(c).unwrap();
    (s, pos) // 所有権を借用しない場合、受け取った文字列を返してあげないと、呼び出し側ではそれを引き続き使えない
}
