fn main() {
    println!("Hello, world!");

    // another_function(5);
    another_function(5, 'h');

    let y = {
        let x = 3;
        // x+1の後ろにセミコロンを入れてしまうと式ではなく文になってしまうので、コンパイルエラーになる。
        x + 1
    };

    println!("The value of y is: {}", y);

    // 戻り値のある関数
    let x = five();
    println!("The value of x is: {}", x);

    // 戻り値のある関数の応用
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

// fn another_function(x: i32) {
//     println!("The value of x is: {}", x);
// }

fn another_function(x: i32, unit_label: char) {
    println!("The measurement is: {}{}", x, unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one (x: i32) -> i32 {
    // これは文として判断されるため、コンパイルエラーになる
    x + 1;
    // x + 1
}
