fn main() {
    wrapper_vec_for_string();
    literal_methods();
    utf8();
    update_string();
    access_index();
    panic_char_slice();
    scan_char();
}

fn wrapper_vec_for_string() {
    let mut _s = String::new();
}

fn literal_methods() {
    let data = "inital contents";

    // やってることは同じこと
    let _s = data.to_string();
    let _s = "inital contents".to_string();
    let _s = String::from("initail contents");
}

fn utf8() {
    // デフォルトutf8なので、すべて有効なStringの値となる。
    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");
}

fn update_string() {
    // push_strメソッドでStringに文字列スライスを追記する
    let mut s = String::from("foo");
    s.push_str("bar");

    // 中身をStringに追加した後に、文字列スライスを使用する
    // push_strメソッドは、引数の所有権を得なくてもいいので、問題なく動作する。
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    // pushでString値に1文字を追加する
    let mut s3 = String::from("lo");
    s3.push('l');

    // +演算子で連結
    let s4 = String::from("Hello, ");
    let s5 = String::from("world!");
    let _s6 = s4 + &s5; // s4はムーブされ、もう使用できなくなる
    // この文は実際にはs4の所有権を奪い、s5の中身のコピーを追記し、結果の所有権を返す
    // つまり、+演算子での連結は、所有権を必ずもらい受ける形になり、他は参照となる
    
    // 複数連結してみる
    let s7 = String::from("tic");
    let s8 = String::from("tac");
    let s9 = String::from("toe");
    let _s10 = s7 + "-" + &s8 + "-" + &s9;
    // これは面倒なので、format!マクロを使用する
    let s11 = String::from("toe");
    let s12 = String::from("toe");
    let s13 = String::from("toe");
    let _s14 = format!("{s11}-{s12}-{s13}");
}

fn access_index() {
    let _s1 = String::from("hello");
    // Stringはインデックスアクセスをサポートしていないため、コンパイルエラーとなる。
    // let h = s1[0];

    // Q. なぜ、サポートしていないか？
    // A. 言語によってスカラーの取りうるバイト数が異なるから。
    //
    // LOL = 3byte -> "LOL"[0] = L
    // Здр = 6byte -> "Здр"[0] = ??
    //
    // 詳しくは、ちゃんと調べること。
}

fn panic_char_slice() {
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    // この文字列は1文字2byteなので、s -> Зд となる。
    println!("{s}");

    // helloは、2byteの集合なので、中途半端でアクセスすると実行時にパニックになる。
    // let s = &hello[0..1];
}

fn scan_char() {
    // 文字列の部分に対して操作を行うための最良の方法は、
    // 文字に対して操作したいのかバイトに対して操作したいのかを明示することです。 
    println!("---- chars");
    for c in "Зд".chars() {
        println!("{c}");
    }
    
    println!("---- bytes");
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
