fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    // '{}'の長さは、{}です
    println!("The length of '{}' is {}.", s1, len);
    
    // これはエラーになる
    // 参照は不変なので、変更できない。
    // let s1 = String::from("hello");
    // cannot_change(&s1)
    
    // これは可変参照なので、エラーにならない
    let mut s1 = String::from("hello");
    can_change(&mut s1);

    // これは可変参照を2つ作成しようとしてるので、エラーになります。
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // スコープが違えば大丈夫 
    let mut s = String::from("hello");
    {
        let _r1 = &mut s;
    } 
    let r2 = &mut s;
    println!("{}", r2);

    compile_error_immutable_and_mutable();

    compile_immutable_and_mutable();
    
    floating_ref();
}

fn calculate_length(s: &String) -> usize { // sはStirngへの参照
    s.len()
} // ここで、sはスコープ外になるけど、参照しているものの所有権を持っているわけではないので、
  // ドロップはされない。

// fn cannot_change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn can_change(some_string: &mut String) {
    some_string.push_str(", world");
}

// これはコンパイルエラーになる 
// r3で可変参照としているが、前段のr1, r2は変更されることがわからない。
fn compile_error_immutable_and_mutable(){
   // let mut s = String::from("hello");
   // let r1 = &s;
   // let r2 = &s;
   // let r3 = &mut s;
   // println!("{}, {}, {}", r1, r2, r3);
}

fn compile_immutable_and_mutable(){
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
    // r1, r2 はこれ以降は使用されないので問題なし
    let r3 = &mut s;
    println!("{}", r3);
}

fn floating_ref(){
    // let reference_to_nothing = dangle();
    let _reference_to_nothing = no_dangle();
}

// fn dangle() -> &String { // dangleはStringへの参照を返す
//     let s = String::from("hello"); // sは新しいString
//     &s // String sへの参照を返す
// } // ここで、sはスコープを抜け、ドロップされる。そのヒープメモリは消される。
//   // そのため、解放されているはずのメモリへの参照を返そうとしてコンパイルエラー

fn no_dangle() -> String {
    let s = String::from("hello"); 
    s
}
