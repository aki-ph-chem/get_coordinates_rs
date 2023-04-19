//! コマンドライン引数を配列として受取り、Resultでラップされた構造体に保持させる

/// インプットファイル名、アウトプットファイル名、原子数を保持する構造体
///
/// clone()を実装している
#[derive(Clone)]
pub struct Config {
    /// インプットファイル名
    pub input_file: String,
    /// アウトプットファイル名
    pub output_file: String,
    /// 原子数
    pub atoms: i32,
}

impl Config {
    /// Result型でラップされたConfigのインスタンスを生成するための関連関数
    ///
    /// env::args().collect()で生成されたコマンドライン引数の配列(`[String]`)を渡してResultでラップした構造体を生成する
    /// 
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        match args.len() {
            1 => Err("No input file"),
            2 => Err("No number of atoms"),
            3 => Err("No output file"),
            _ => {
                let input_file = args[1].clone();
                let output_file = args[3].clone();
                let atoms:i32 = args[2].parse().expect("number of atoms is invalid"); 
                Ok(Config {input_file, output_file, atoms})
            }
        }
    }
}
