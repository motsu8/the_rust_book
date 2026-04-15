fn main() {
    hash_function();
    update_or_insert_value();
    entry_or_insert_hashmap();
    overwrite_hashmap();
    ownership_hashmap();
    scan_hashmap();
    get_value();
    create_hashmap();
}

fn create_hashmap() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}

fn get_value() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let _score = scores.get(&team_name).copied().unwrap_or(0);
}

fn scan_hashmap() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

fn ownership_hashmap() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // Stringのような、所有権のある値なら、値はムーブされ、HashMapが所有者となる。
    // つまり、insertした時点で、field_name, field_valueは無効となる。
}

fn overwrite_hashmap() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20);

    println!("{:?}", scores);
    // -> {"Blue": 20}
}

fn entry_or_insert_hashmap() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
    // -> {"Yellow": 50, "Blue": 10}
}

fn update_or_insert_value() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
    // -> {"hello": 1, "world": 2, "wonderful": 1}

    // &xxx は住所を渡す
    // *xxx は住所を見て、家に行く
}

fn hash_function() {
    // HashMapはデフォルトだと、SipHashと呼ばれる、安全性の高い、ハッシュ関数が使用される
    // ドメインによって遅いと判断できる場合、安全性を低め、速度を得たい場合、
    // 異なるhasherを指定することで、ハッシュ関数を切り替えることができる
}
