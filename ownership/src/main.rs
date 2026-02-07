fn main() {
    // 文字列リテラル -> 変更不可
    let s = "hello";

    // String型 -> 変更可
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}

