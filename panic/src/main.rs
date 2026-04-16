use std::fs::File;
use std::io::ErrorKind;

fn main() {
    panic_fnc();
    // result();
    // match_condition();
    // refactor_match_condition();
    // unwrap();
    expect();
}

fn panic_fnc() {
    // クラッシュして炎上
    // panic!("crush and burn");
    //
    // 有効な範囲外にアクセスしてクラッシュ
    // let v = vec![1, 2, 3];
    // v[100];
}

fn result() {
    let greeting_file_result = File::open("hello.txt");
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("ファイルを開くのに問題がありました：{:?}", error),
    };
}

fn match_condition() { 
    let greeting_file_result = File::open("hello.txt");
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("ファイルを作成するのに問題がありました。: {:?}", e),
            },
            other_error => {
                panic!("ファイルを開くのに問題がありました。：{:?}", other_error);
            }
        }
    };
}

fn refactor_match_condition() { 
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn unwrap() {
    let _greeting_file = File::open("hello.txt").unwrap();
}

fn expect() {
    let _greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
