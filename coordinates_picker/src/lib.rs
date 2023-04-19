//! メインの機能は`run()`に実装している。 
//! 
use std::error::Error;
use std::fs::File;
use std::io::Write;

// コマンドライン引数を読み取る
pub mod get_args;
pub use crate::get_args::Config;

// テキストを読んでキーワードを探し、印を付ける
pub mod key_position; 
pub use  crate::key_position::KeyPosition;

// 正規表現の処理
pub mod split_line;
pub use crate::split_line::*;

/// モジュール`get_args`,`key_position`,`split_line`によって
/// 実装されている中心的な機能
///
/// `main()`関数ではコマンドライン引数の読み取り、構造体`Config`の
/// 実装が必要
///
/// 構造体 Configを渡し、Resultでラップされた`String`型の二次元配列
/// `Vec<Vec<String>>`を返す
///
///
pub fn run(config: Config) -> Result<Vec<Vec<String>>,Box<dyn Error>> {
    let key = "Standard orientation:";
    let key_position = KeyPosition::new(config.input_file, key);

    let key_position_max = key_position.key_position.iter()
        .rev()
        .next()
        .unwrap();

    //println!("key_point_max + 6 = {}", key_position_max + 6);
    let mut vec_string: Vec<Vec<String>> = vec![];
    for i in (key_position_max + 5)..(config.atoms as usize + key_position_max + 5) {
        //println!("key_position.lines[{}] = {}",i, key_position.lines[i]);
        vec_string.push(get_num_string(key_position.lines[i].clone()));
    }

    Ok(vec_string)
}

/// `run()`で生成した二次元配列`Vec<Vec<String>>`を出力して確認するための関数 
///
pub fn print_2d_vec_test(vec_2d: Vec<Vec<String>>) {
    for v in vec_2d {
        println!("v = {:?}", v);
    } 
}

/// `run()`で生成した二次元配列`Vec<Vec<String>>`を*csvとしてファイルに出力するための関数
/// 
///
pub fn out_put_file(config: Config, coo: Vec<Vec<String>>) {
    let mut file = File::create(config.output_file).expect("file not found");
        for vec in coo {
            writeln!(file, "{}, {}, {}, {}, {}, {}",
                     vec[0], vec[1], vec[2], vec[3], vec[4], vec[5]);
        }
}
