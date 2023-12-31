pub mod ast;
pub mod lexer;
pub mod parser;
pub mod token;

use lexer::*;
use parser::parse;
use std::io::{stdin, stdout, BufRead, BufReader, Result, Write};

// プロンプトを表示しユーザーの入力を促す
fn prompt(s: &str) -> Result<()> {
    let stdout = stdout();
    let mut stdout = stdout.lock();
    stdout.write_all(s.as_bytes())?;
    stdout.flush()
}

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();
    let stdin = BufReader::new(stdin);
    let mut lines = stdin.lines();

    loop {
        prompt(">").unwrap();
        // ユーザーの入力を取得する
        if let Some(Ok(line)) = lines.next() {
            // 字句解析を行う
            let tokens = lex(&line).unwrap();
            // 字句解析した結果をパースし
            let ast = parse(tokens).unwrap();
            // 出力する
            println!("{:?}", ast);
        } else {
            break;
        }
    }
}
