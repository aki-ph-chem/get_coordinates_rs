//! ファイルのパスからファイルを一行ずつ読み込み、
//! そのファイルの特定のキーワードが含まれる場合はその場所を保持する
//!

use std::fs::File;
use std::io::{BufRead,BufReader};

/// テキストファイルを下処理して結果を各フィールドのベクタに保持させるための構造体
///
pub struct KeyPosition {
/// キーワードの存在した行の位置
    pub key_position: Vec<usize>,
/// キーワードの存在した行の内容
    pub key_line: Vec<String>,
/// 全ての行 
    pub lines: Vec<String>,
}

impl KeyPosition {
    /// KeyPositionのインスタンスを生成するための関連関数
    ///
    /// インプットファイル名とキーワードを渡して、構造体を生成する
    ///
    pub fn new(path: String, keyword: &str) -> KeyPosition {
        let file = File::open(path).expect("file not found");
        // storages
        let mut key_position = Vec::new();
        let mut key_line = Vec::new();
        let mut lines = Vec::new();
        // for
        for (counter, f) in BufReader::new(file).lines().enumerate(){
            if let Ok(line) = f{
                lines.push(line.clone());
                if line.contains(keyword){
                    key_position.push(counter);
                    key_line.push(line);
                }
            }
        }
        KeyPosition{key_position, key_line, lines}
    }
}

#[cfg(test)]
mod test_key_position {
    use super::*;
    use KeyPosition;

    #[test]
    fn test_key_position_new(){
        let ans_key_line = vec![
            "hoge aa".to_string(),
            "hoge bb".to_string(),
            "hoge ee".to_string(),
        ];

        let ans_position = vec![0, 4, 7];

        let file = File::open("for_test/test_2").expect("file not found");
        let mut ans_lines = Vec::new();
        for f in BufReader::new(file).lines() {
            if let Ok(line) = f{
                ans_lines.push(line);
            }
        }

        let res = KeyPosition::new("for_test/test_2".to_string(), "hoge"); 
        assert_eq!(res.key_position, ans_position);
        assert_eq!(res.key_line, ans_key_line);
        assert_eq!(res.lines, ans_lines);
    }
}
