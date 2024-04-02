fn main() {
    let s = String::from("Hello, world!");
    function_move(s);
    println!("sは「{}」です。", s); // function_moveの終了時にsが破棄されるので、sを使えない
}

fn function_move(m: String) {
    // 非スカラー型を引数として渡すと所有権がmに移動する
    println!("function_move: 引数mの値は「{}」です。", m);
    // 関数が終了するとmが破棄され、対象のメモリ領域も解放されるので、呼び出し元で変数を使用できない
}
