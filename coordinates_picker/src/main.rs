//! このバイナリクレートについて
//!
//! このバイナリクレートでは、GaussianのLOGファイルから構造最適化
//! された座標を*.csvとして取り出す
//!
//! # Example
//!
//! ```bash
//! $ co_pick <file name of LOG> <number of atoms> <file name of output(*.csv) >
//! ```
use std::env;
use std::process;

extern crate coordinates_picker as c_rs;

///
/// コマンドライン引数のベクタ`args`を`Config::new()`に渡し構造体`Config`のインスタンスを生成する
///
/// このインスタンス`config`を`run()`に渡して、二次元の`String`型のベクタ`Vec<Vec<String>>`を生成する
///
/// この二次元のベクタを`out_put_to_file`に渡し、ファイルもしくは標準出力に出力する
fn main(){
    let args: Vec<_> = env::args().collect();
    let config = c_rs::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem while parsing arguments: {}", err);
        process::exit(1);
    });

    let coordinates = match c_rs::run(config.clone()) {
        Ok(vec) => vec,
        Err(e) => {
            eprintln!("Apprication error: {}", e);
            process::exit(1);
        },
    };

    if let Err(e) = c_rs::out_put_to_file(config, &coordinates) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
