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
    
    /*
     * ディープコピー
     */
    // これはヒープがコピーされる
    let s2 = _s2.clone();
    println!("s1 = {}, s2 = {}", _s2, s2);
    

    /*
     * 所有権と関数
     */
    let ss = String::from("hello"); // ssがスコープに入る
    takes_ownership(ss); // ssの値が関数にムーブされるので、これ以降は使えない
    // println!("{}", ss);
    
    let x = 5; // xがスコープに入る
    makes_copy(x); // xも関数にムーブされるが、i32はCopyなので、この後にxを使っても大丈夫
    println!("{}", x);
    
    /*
     * 戻り値とスコープ
     */
    return_and_scope();
}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}

/*
 * 戻り値とスコープ
 */
fn return_and_scope(){
    let _s1 = gives_ownership();
    let s2 = String::from("hello");
    let _s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

