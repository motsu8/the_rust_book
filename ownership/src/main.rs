fn main() {
    // 文字列リテラル -> 変更不可
    let _s = "hello";

    // String型 -> 変更可
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    /*
     * ムーブによる変数とデータの相互作用
     */
    // 固定サイズの単純な値
    let x = 5;
    let _y = x;

    // 可変サイズの値
    let s1 = String::from("hello");
    let _s2 = s1;
    // s1の所有権確認 -> これはヒープの所有権がs2に移っているので、使えない。（コンパイルエラー）
    // シャローコピーっぽいが、元の変数は使えないので、Rustではムーブと表現している。
    // シャローコピーは元の変数もそのまま使用できる。
    // println!("{}, world!", s1);
}

