//! このモジュールでは正規表現を処理するクレート`Regex`を用いて、
//! 文字列(ファイルの一行)から数値の部分を`Vec<String>`として取り出す
//!

use regex::Regex;

/// 正規表現も用いて文字列から数値の部分を切り取り`Vec<String>`として返す
///
/// # Example
///
/// ```
/// let s_num = "3 12 1.2  2.3  4.0".to_string();
/// let array: Vec<_> = ["3", "12","1.2", "2.3", "4.0"]
///    .iter().map(|s| s.to_string()).collect();
///
/// let res = coordinates_picker::get_num_string(s_num);
/// for i in 0..array.len() {
///    assert_eq!(res[i], array[i]);
///    }
/// ```
///
pub fn get_num_string(s: String) -> Vec<String> {
    let regex = Regex::new(r"[-0-9.]+");

    //let mut array = Vec::<String>::new();
    let mut array = Vec::new();
    for caps in regex.expect("REASON").captures_iter(s.as_str()) {
        array.push(caps.get(0).unwrap().as_str().to_string());
    }
    array
}

#[cfg(test)]
mod test_split {
    use std::fs::File;
    use std::io::{BufRead,BufReader};
    use super::*;

    #[test]
    fn test_get_num_string() {
        let s_num = "3 12 1.2  2.3  4.0".to_string();
        let array: Vec<_> = ["3", "12","1.2", "2.3", "4.0"]
            .iter().map(|s| s.to_string()).collect();

        let res = get_num_string(s_num);
        for i in 0..array.len() {
            assert_eq!(res[i], array[i]);
        }
    }

    #[test]
    fn test_get_num_for_coodinates() {
        let coo_string: Vec<_> =  [
"1 6 0 -0.042214 1.387483 -0.124879",
"2 6 0 -1.373387 0.677396 0.071920",
"3 6 0 -1.373402 -0.677370 0.071909",
"4 6 0 -0.042245 -1.387488 -0.124878",
"5 1 0 0.021754 1.869823 -1.110964",
"6 1 0 0.144642 2.156219 0.637581",
"7 1 0 -2.286675 1.266023 0.171891",
"8 1 0 -2.286704 -1.265977 0.171869",
"9 1 0 0.021727 -1.869859 -1.110946",
"10 1 0 0.144590 -2.156201 0.637611",
"11 15 0 1.320856 -0.000008 -0.013380",
"12 1 0 1.415322 -0.000024 1.439234",
        ].iter().map(|s| s.to_string()).collect();
        let mut coo_ans: Vec<Vec<String>> = vec![];
        for s in coo_string {
            coo_ans.push(get_num_string(s));
        }

        let file = File::open("for_test/test_coo".to_string()).expect("file not found");
        let mut coo: Vec<Vec<String>> = vec![];
        for f in BufReader::new(file).lines() {
            if let Ok(line) = f {
                coo.push(get_num_string(line));
            }
        }
        assert_eq!(coo, coo_ans);
    }
}
