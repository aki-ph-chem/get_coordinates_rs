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

    //c_rs::print_2d_vec_test(coordinates);
    c_rs::out_put_file(config, coordinates); 
}
