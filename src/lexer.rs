use crate::token::*;

// 字句解析器
pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
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
        match input[position] {
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
            b => {
                return Err(LexError::invalid_char(
                    b as char,
                    Location(position, position + 1),
                ))
            }
        }
    }
    Ok(tokens)
}

pub fn consume_byte(input: &[u8], position: usize, b: u8) -> Result<(u8, usize), LexError> {
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
    consume_byte(input, start, b'+').map(|(_, end)| (Token::plus(Location(start, end)), end))
}

fn lex_minus(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'-').map(|(_, end)| (Token::minus(Location(start, end)), end))
}

fn lex_asterisk(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'*').map(|(_, end)| (Token::asterisk(Location(start, end)), end))
}

fn lex_slash(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'/').map(|(_, end)| (Token::slash(Location(start, end)), end))
}

fn lex_lparen(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'(').map(|(_, end)| (Token::lparen(Location(start, end)), end))
}

fn lex_rparen(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b')').map(|(_, end)| (Token::rparen(Location(start, end)), end))
}

fn lex_number(input: &[u8], position: usize) -> Result<(Token, usize), LexError> {
    use ::std::str::from_utf8;

    let start = position;
    let end = recognize_many(input, start, |b| b"1234567890".contains(&b));
    let n = from_utf8(&input[start..end]).unwrap().parse().unwrap();
    Ok((Token::number(n, Location(start, end)), end))
}

// テストがループしていた理由は -- b" \n\t" -- が -- b"\n\t" -- になっていた。
fn skip_spaces(input: &[u8], position: usize) -> Result<((), usize), LexError> {
    let position = recognize_many(input, position, |b| b" \n\t".contains(&b));
    Ok(((), position))
}

fn recognize_many(input: &[u8], mut position: usize, mut f: impl FnMut(u8) -> bool) -> usize {
    while position < input.len() && f(input[position]) {
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
        let input = "123";
        let test_input = input.as_bytes();
        let test_position = 2;
        let expected_byte = b'3';
        let expected_results = Ok((expected_byte, test_position + 1));
        let result = consume_byte(test_input, test_position, expected_byte);
        assert!(result.is_ok());
        assert_eq!(expected_results, result);

        // エラーケース1
        let unexpected_byte = b'1';
        let expected_results2 = Err(Annotation {
            value: LexErrorKind::InvalidChar('3'),
            loc: Location(2, 3),
        });
        let result = consume_byte(test_input, test_position, unexpected_byte);
        assert!(result.is_err());
        assert_eq!(expected_results2, result);

        // エラーケース２
        let expected_byte = b'3';
        let unexpected_position = 3;
        let expected_results3 = Err(Annotation {
            value: LexErrorKind::Eof,
            loc: Location(3, 3),
        });
        let result = consume_byte(test_input, unexpected_position, expected_byte);
        assert!(result.is_err());
        assert_eq!(expected_results3, result);
    }

    #[test]
    fn test_lex_plus() {
        let input = "123+";
        let test_input = input.as_bytes();
        let test_position = 3;
        let expected_results = Ok((
            Annotation {
                value: TokenKind::Plus,
                loc: Location(3, 4),
            },
            test_position + 1,
        ));
        let result = lex_plus(test_input, test_position);
        assert!(result.is_ok());
        assert_eq!(expected_results, result);
    }

    #[test]
    fn test_lex_minus() {
        let input = "123-";
        let test_input = input.as_bytes();
        let test_position = 3;
        let expected_results = Ok((
            Annotation {
                value: TokenKind::Minus,
                loc: Location(3, 4),
            },
            test_position + 1,
        ));
        let result = lex_minus(test_input, test_position);
        assert!(result.is_ok());
        assert_eq!(expected_results, result);
    }

    #[test]
    fn test_lex_asterisk() {
        let input = "123*";
        let test_input = input.as_bytes();
        let test_position = 3;
        let expected_results = Ok((
            Annotation {
                value: TokenKind::Asterisk,
                loc: Location(3, 4),
            },
            test_position + 1,
        ));
        let result = lex_asterisk(test_input, test_position);
        assert!(result.is_ok());
        assert_eq!(expected_results, result);
    }

    #[test]
    fn test_lex_slash() {
        let input = "123/";
        let test_input = input.as_bytes();
        let test_position = 3;
        let expected_results = Ok((
            Annotation {
                value: TokenKind::Slash,
                loc: Location(3, 4),
            },
            test_position + 1,
        ));
        let result = lex_slash(test_input, test_position);
        assert!(result.is_ok());
        assert_eq!(expected_results, result);
    }

    #[test]
    fn test_lex_lparen() {
        let input = "123(";
        let test_input = input.as_bytes();
        let test_position = 3;
        let expected_results = Ok((
            Annotation {
                value: TokenKind::Lparen,
                loc: Location(3, 4),
            },
            test_position + 1,
        ));
        let result = lex_lparen(test_input, test_position);
        assert!(result.is_ok());
        assert_eq!(expected_results, result);
    }

    #[test]
    fn test_lex_rparen() {
        let input = "123)";
        let test_input = input.as_bytes();
        let test_position = 3;
        let expected_results = Ok((
            Annotation {
                value: TokenKind::Rparen,
                loc: Location(3, 4),
            },
            test_position + 1,
        ));
        let result = lex_rparen(test_input, test_position);
        assert!(result.is_ok());
        assert_eq!(expected_results, result);
    }

    #[test]
    fn test_lex_number() {
        let input = "1235()";
        let test_input = input.as_bytes();
        let test_position = 0;
        let expected_results = Ok((
            Annotation {
                value: TokenKind::Number(1235),
                loc: Location(0, 4),
            },
            4,
        ));
        let result = lex_number(test_input, test_position);
        assert!(result.is_ok());
        assert_eq!(expected_results, result);
    }

    #[test]
    fn test_skip_spaces() {
        let input = " \n\t123";
        let test_input = input.as_bytes();
        let test_position = 0;
        let expected_results = Ok(((), 3));
        let result = skip_spaces(test_input, test_position);
        assert!(result.is_ok());
        assert_eq!(expected_results, result);
    }

    #[test]
    fn test_recognize_many() {
        let input = "4789+++";
        let test_input = input.as_bytes();
        let test_position = 2;
        let test_fn_contains = |x| b"1234567890".contains(&x);
        let expected_result = 4;
        let result = recognize_many(test_input, test_position, test_fn_contains);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_lexer() {
        assert_eq!(
            lex("1 + 2 * 3 - - 10"),
            Ok(vec![
                Token::number(1, Location(0, 1)),
                Token::plus(Location(2, 3)),
                Token::number(2, Location(4, 5)),
                Token::asterisk(Location(6, 7)),
                Token::number(3, Location(8, 9)),
                Token::minus(Location(10, 11)),
                Token::minus(Location(12, 13)),
                Token::number(10, Location(14, 16)),
            ])
        )
    }
}
