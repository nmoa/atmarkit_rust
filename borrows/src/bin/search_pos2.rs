fn main() {
    let s = String::from("Hello, world!!");
    let c = 'w';

    let pos = search_position(&s, c);
    println!("文字'{}'の「{}」中の位置は{}文字目です。", c, s, pos);
}

// 参照型にすると値の使用のみが可能になる。
// 関数内部で変更されないことが保証されるため、所有権を移動させる必要がなくなる。
fn search_position(s: &String, c: char) -> usize {
    let pos = s.find(c).unwrap();
    pos
}
