use std::io;

fn main() {
    let _guess: u32  = "42".parse().expect("Not a number!");
    
    let _x = 2.0; // f63

    let _y: f32 = 3.0; // f32

    // 足し算
    let _sum = 5 + 10;

    // 引き算
    let _difference = 95.5 - 4.3;

    // 掛け算
    let _product = 4 * 30;

    // 割り算
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3; // 結果は0になる。

    // あまり
    let _remainder = 43 % 5;

    // 論理値
    let _t = true;
    let _f: bool = false; // 明示的型注釈付き

    // 文字型 char ユニコード
    let _c = 'z';

    // タプル型
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    // 6.4
    println!("The value of y is: {}", y);
    
    let t: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = t.0;     // 500
    let _six_point_four = t.1;   // 6.4
    let _one = t.2;              // 1

    // 配列-> 固定配列
    let _a = [1, 2, 3, 4, 5];
    let _a: [i32; 5] = [1,2,3,4,5];
    let _a = [3; 5];     // [3,3,3,3,3]

    let a = [1, 2, 3, 4, 5];
    let _first = a[0];
    let _second = a[1];

    /*
     * 配列の終端を超えて要素にアクセスしたときの挙動確認
     */
    let a = [1,2,3,4,5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index enterd was not a number.");

    let element = a[index];
    
    println!("The value of the element at index {} is: {}", index, element);
}
