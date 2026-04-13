fn main() {
    // 空のベクタを作成してみる
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);

    // マクロで推論されるベクタを作成してみる
    // 整数の型はデフォルトでi32なので、wの型は、Vec<i32>となる。
    let w = vec![1, 2, 3];
    println!("{:?}", w);

    update_vec();
    read_vec();
    process_vec();
    enum_vec();
    drop_vec();
}

fn update_vec() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);
    v.push(0);

    println!("{:?}", v);
}

fn read_vec() {
    // 添え字記法
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("3つ目の要素は{third}です");

    // getメソッド
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("3つ目の要素は{third}です"),
        None => println!("3つ目の要素はありません"),
    }

    // これはリスト範囲外の参照となるため、パニックを起こす
    // let does_not_exist = &v[100];

    // これはパニックを起こすことなく、Noneを返す。
    let _does_not_exist = v.get(100);

    // これはコンパイルエラー
    // ベクタはメモリ上に値同士を隣り合うように配置するため、
    // ベクタの終端に追加するとき、スペースがない場合、
    // 新しいメモリを割り当て、古い要素を新しいスペースにコピーすることになる。
    // その場合、最初の要素を指す参照は、解放されたメモリを指すことになる。
    // これが起こらないように借用規則で防いでくれている。
    let mut w = vec![1, 2, 3, 4, 5];
    let first = &w[0];
    // w.push(6);
    println!("最初の要素は: {first}");
}

fn process_vec() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // 各要素に50足している
    // 可変参照が参照している値を変更するには、+=演算子を使用する前に、
    // *参照外し演算子を使用して、iの値にたどり着く必要がある。
    let mut w = vec![90, 80, 70];
    for i in &mut w {
        *i += 50;
    }
    println!("{:?}", w)
}

fn enum_vec() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);
}

fn drop_vec() {
    {
        let v = vec![1, 2, 3, 4];
        for i in v {
            println!("{i}")
        }
    } //　<- vはここでスコープを抜け、解放される
}
