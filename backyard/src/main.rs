// ここのuseキーワードでパスを定義して、これ以降では、Asparagusを記載するだけで使用できる。
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("Im growing {:?}!", plant);
}
