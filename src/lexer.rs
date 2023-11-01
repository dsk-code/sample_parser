use crate::token::*;

// 字句解析器
pub fn lex(input: &str) -> Result<Vec<Token>, LexError>{
    // 解析結果を保存するベクタ
    let mut tokens = Vec::new();
    // 入力
    let input = input.as_bytes();
    // 位置を管理する値
    let mut position = 0;
    // サブレキサを呼んだ後posを更新するマクロ
    macro_rules! lex_a_token {
        ($lexer:expr) => {{
            let (tok, p) = $lexer?;
            tokens.push(tok);   
            position = p;
        }};
    }
    while position < input.len() {
        // ここでそれぞれの関数にinputとposを渡す
        match input[position]{
            // 遷移図通りの実装
            b'0'..=b'9' => lex_a_token!(lex_number(input, position)),
            b'+' => lex_a_token!(lex_plus(input, position)),
            b'-' => lex_a_token!(lex_minus(input, position)),
            b'*' => lex_a_token!(lex_asterisk(input, position)),
            b'/' => lex_a_token!(lex_slash(input, position)),
            b'(' => lex_a_token!(lex_lparen(input, position)),
            b')' => lex_a_token!(lex_rparen(input, position)),
            // 空白を扱う
            b' ' | b'\n' | b'\t' => {
                let ((), p) = skip_spaces(input, position)?;
                position = p;
            }
            // それ以外が来たらエラー
            b => return Err(LexError::invalid_char(b as char, Location(position, position + 1))),
        }
    }
    Ok(tokens)
}

pub fn consume_byte(input: &[u8], position: usize, b: u8) -> Result<(u8, usize), LexError>{
    if input.len() <= position {
        return Err(LexError::eof(Location(position, position)));
    }
    if input[position] != b {
        return Err(LexError::invalid_char(
            input[position] as char, 
            Location(position, position + 1),
        ));
    }

    Ok((b, position + 1))
}

fn lex_plus(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'+')
    .map(|(_, end)|(Token::plus(Location(start, end)), end))
}

fn lex_minus(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'-')
    .map(|(_, end)|(Token::minus(Location(start, end)), end))
} 

fn lex_asterisk(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'*')
    .map(|(_, end)|(Token::asterisk(Location(start, end)), end))

}

fn lex_slash(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'/')
    .map(|(_, end)|(Token::slash(Location(start, end)), end))
}

fn lex_lparen(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'(')
    .map(|(_, end)|(Token::lparen(Location(start, end)), end))
}

fn lex_rparen(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b')')
    .map(|(_, end)|(Token::rparen(Location(start, end)), end))
}

fn lex_number(input: &[u8], position: usize) -> Result<(Token, usize), LexError> {
    use::std::str::from_utf8;

    let start = position;
    let end = recognize_many(input, start, |b|b"1234567890".contains(&b));
    let n = from_utf8(&input[start..end])
        .unwrap()
        .parse()
        .unwrap();
    Ok((Token::number(n, Location(start, end)), end))
}

fn skip_spaces(input: &[u8], position: usize) -> Result<((),usize), LexError> {
    let position = recognize_many(input, position, |b| b"\n\t".contains(&b));
    Ok(((), position))
}

fn recognize_many(input: &[u8], mut position: usize, mut f: impl FnMut(u8) -> bool) -> usize {
    while position < input.len() && f(input[position]){
        position += 1;
    }
    position
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consume_byte() {
        // 正常ケース
        let test_input = "123";
        let input = test_input.as_bytes();
        assert_eq!([49, 50, 51], input);
        let test_position = 2;
        let expected_byte = b'3';
        let expected_results = (51, 3);
        let result = consume_byte(input, test_position, expected_byte);
        assert!(result.is_ok());
        assert_eq!(expected_results, result.unwrap());

        // エラーケース
    }

    #[test]
    fn test_lex_plus() {
        let test_input = "123+";
        let input = test_input.as_bytes();
        assert_eq!([49, 50, 51, 43], input);
        let test_position = 3;
        let result = lex_plus(input, test_position);
        assert!(result.is_ok());
        let (result_annotation, result_position) = result.unwrap();
        assert_eq!(Annotation { value: TokenKind::Plus, loc: Location(3, 4)}, result_annotation);
        assert_eq!(4, result_position);
    }

    #[test]
    fn test_lex_minus() {
        let test_input = "123-";
        let input = test_input.as_bytes();
        assert_eq!([49, 50, 51, 45], input);
        let test_position = 3;
        let result = lex_minus(input, test_position);
        assert!(result.is_ok());
        let (result_annotation, reslt_position) = result.unwrap();
        assert_eq!(Annotation { value: TokenKind::Minus, loc: Location(3, 4)}, result_annotation);
        assert_eq!(4, reslt_position);
    }

    #[test]
    fn test_lex_asterisk() {
        let test_input = "123*";
        let input = test_input.as_bytes();
        assert_eq!([49, 50, 51, 42], input);
        let test_position = 3;
        let result = lex_asterisk(input, test_position);
        assert!(result.is_ok());
        let (result_annotation, result_position) = result.unwrap();
        assert_eq!(Annotation { value: TokenKind::Asterisk, loc: Location(3, 4)}, result_annotation);
        assert_eq!(4, result_position);
    }
    // #[test]
    // fn test_lex_a_token_macro() {
    //     let test_input = "123+";
        
    //     fn dummy_fnction(input: &[u8], position: usize) -> Result<(Token, usize), LexError> {
            
    //     }

    // }
    // #[test]
    // fn test_lexer() {
    //     assert_eq!(
    //         lex("1 + 2 * 3 - - 10"),
    //         Ok(vec![
    //             Token::number(1, Location(0, 1)),
    //             Token::plus(Location(2, 3)),
    //             Token::number(2, Location(4, 5)),
    //             Token::asterisk(Location(6, 7)),
    //             Token::number(3, Location(8, 9)),
    //             Token::minus(Location(10, 11)),
    //             Token::minus(Location(12, 13)),
    //             Token::number(10, Location(14, 16)),
    //         ])
    //     )
    // }
}
