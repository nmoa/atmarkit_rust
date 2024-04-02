fn main() {
    let x = 1;
    let y = x; // スカラー型は所有権を複製する(≒変数のコピー)
    println!("xは{}です。", x);
    println!("yは{}です。", y);
}
