use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CurrencyCode {
    code: String,
    decimals: Option<i32>,
    name: String,
    number: String,
}

fn main() {
    // 工作目录为当前项目目录
    let file_path = Path::new("src/meta/currency_codes.json");

    // Match 一下, 找不到文件的话, 程序要崩
    match File::open(file_path) {
        Ok(file) => {
            let currency_codes: Vec<CurrencyCode> =
                serde_json::from_reader(file).expect("error while reading");
            for currency_code in currency_codes {
                println!(
                    "Code: {}, Decimals: {}, name: {}, number: {}",
                    currency_code.code,
                    currency_code.decimals.unwrap_or(0),
                    currency_code.name,
                    currency_code.number
                )
            }
        }
        Err(err) => {
            println!("my error message: {}", err);
        }
    };
}
