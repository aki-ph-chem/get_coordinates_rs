//! メインの機能は`run()`に実装している。 
//! 
use std::fs::File;
use std::io::{Write,Error};

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
pub fn run(config: Config) -> Result<Vec<Vec<String>>,&'static str>{
    let key = "Standard orientation:";
    let key_position = KeyPosition::new(config.input_file, key);

    let mut vec_string: Vec<Vec<String>> = vec![];
    if let Some(key_position_max) = key_position.key_position.iter().rev().next() 
            {
                for i in (key_position_max + 5)..(config.atoms as usize + key_position_max + 5) {
                    vec_string.push(get_num_string(key_position.lines[i].clone()));
                }
            } else {
                return Err("keyword `Standard orientation:` is not found in this file");
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
pub fn out_put_to_file(config: Config, coo: &Vec<Vec<String>>) -> Result<(),Error> {
    if let Some(output_file) = &config.output_file {
        let mut file = File::create(output_file).expect("file not found");
        for vec in coo {
            writeln!(file, "{},{},{},{},{},{}",
                     vec[0], vec[1], vec[2], vec[3], vec[4], vec[5])?;
        }
    }else{
        for vec in coo {
            println!("{},{},{},{},{},{}",
                     vec[0], vec[1], vec[2], vec[3], vec[4], vec[5]);
        }
    }
    Ok(())
}
