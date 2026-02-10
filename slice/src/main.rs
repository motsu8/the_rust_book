fn main() {
    // let mut s = String::from("hello world");
    // let word = first_word(&s);  // wordの中身は、値5になる
    // s.clear();                  // Stringを空にする。つまり、""と等しくなる
    // // wordはまだ値5を保持しているが、もうこの値を正しい意味で使用できる文字列は存在しない。
    // // wordは今や完全に無効"

    /* 
     * 文字スライス
     */
    let w = String::from("hello world");
    let _hello = &w[0..5];       // hello
    let _world = &w[6..11];      // world

    let _slice = &w[0..2];
    // ↑同じ
    let _slice = &w[..2]; 

    let len = w.len();
    let _slice = &w[3..len];
    // ↑同じ
    let _slice = &w[3..];

    let _slice = &w[0..len];
    // ↑同じ
    let _slice = &w[..];

    /*
     * first_wordの論理エラーをなくすリファクタ
     */
    // let mut s = String::from("hello world");
    // let word = first_word(&s);
    // s.clear();  // error
    // println!("The first word is: {}", word);


    /*
     * 参照外し型強制
     */
    let my_string = String::from("hello world");
    
    // first_wordはStringの全体または部分へのスライスに対して機能する
    let _word = first_word(&my_string[0..6]);
    let _word = first_word(&my_string[..]);
    // first_wordはStringの参照に対しても機能する。この場合は、
    // String全体へのスライスと同等
    let _word = first_word(&my_string);

    let my_string_literal = "hello world";

    // first_wordは文字列リテラルの全体または部分へのスライスに対して機能する
    let _word = first_word(&my_string_literal[0..6]);
    let _word = first_word(&my_string_literal[..]);

    // 文字列リテラルは「それ自体すでに文字列リテラルなので」、スライス機能なしでも機能する
    let _word = first_word(my_string_literal);

    array_slice();
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
// 
//     for (i, &item) in bytes.iter().enumerate(){
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn array_slice(){
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}
