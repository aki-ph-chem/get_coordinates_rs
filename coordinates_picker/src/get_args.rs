//! コマンドライン引数を配列として受取り、Resultでラップされた構造体に保持させる

/// インプットファイル名、アウトプットファイル名、原子数を保持する構造体
///
/// clone()を実装している
#[derive(Clone,PartialEq,Debug)]
pub struct Config {
    /// インプットファイル名
    pub input_file: String,
    /// アウトプットファイル名 
    pub output_file: Option<String>,
    /// 原子数
    pub atoms: i32,
}

impl Config {
    /// Result型でラップされたConfigのインスタンスを生成するための関連関数
    ///
    /// env::args().collect()で生成されたコマンドライン引数の配列(`[String]`)を渡してResultでラップした構造体を生成する
    /// 
    /// 第3引数がある場合では、`Optinon<String>`型のoutput_fileは`Some`で、ない場合では`None`
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        match args.len() {
            1 => Err("No input file"),
            2 => Err("No number of atoms"),
            3 => {
                let input_file = args[1].clone();
                let atoms:i32 = args[2].parse().expect("number of atoms is invalid"); 
                Ok(Config {input_file: input_file, output_file: None, atoms: atoms})
            },
            _ => {
                let input_file = args[1].clone();
                let output_file = args[3].clone();
                let atoms:i32 = args[2].parse().expect("number of atoms is invalid"); 
                Ok(Config {input_file: input_file, output_file: Some(output_file), atoms: atoms})
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_config_new_3args() -> Result<(), &'static str> {
        let args: Vec<String> = ["coordinates_picker", "input_file", "12", "output_file"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let config_3args = Config::new(&args)?;
        let config_3args_ans = Config {
            input_file: "input_file".to_string(),
            output_file: Some("output_file".to_string()),
            atoms: 12,
        };

        assert_eq!(config_3args, config_3args_ans);
        Ok(())
    }

    #[test]
    fn test_config_new_2args() -> Result<(), &'static str> {
        let args: Vec<String> = ["coordinates_picker", "input_file", "12"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let config_2args = Config::new(&args)?;
        let config_2args_ans = Config {
            input_file: "input_file".to_string(),
            output_file: None,
            atoms: 12,
        };

        assert_eq!(config_2args, config_2args_ans);
        Ok(())
    }
}
