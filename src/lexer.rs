use crate::token::*;

pub fn lex(input: &str) -> Result<Vec<Token>, LexError>{
    let mut tokens = Vec::new();
    let input = input.as_bytes();
    let mut pos = 0;

    macro_rules! lex_a_token {
        ($lexer:expr) => {{
            let (tok, p) = $lexer?;
            tokens.push(tok);
            pos = p;
        }};
    }
    while pos < input.len() {
        match input[pos]{
            b'0'..=b'9' => lex_a_token!(lex_number(input, pos)),
            b'+' => lex_a_token!(lex_plus(input, pos)),
            b'-' => lex_a_token!(lex_minus(input, pos)),
            b'*' => lex_a_token!(lex_asterisk(input, pos)),
            b'/' => lex_a_token!(lex_slash(input, pos)),
            b'(' => lex_a_token!(lex_lparen(input, pos)),
            b')' => lex_a_token!(lex_rparen(input, pos)),
            b' ' | b'\n' | b'\t' => {
                let ((), p) = skip_spaces(input, pos)?;
                pos = p;
            }
            b => return Err(LexError::invalid_char(b as char, Location(pos, pos + 1))),
        }
    }
    Ok(tokens)
}

pub fn consume_byte(input: &[u8], pos: usize, b: u8) -> Result<(u8, usize), LexError>{
    if input.len() <= pos {
        return Err(LexError::eof(Location(pos, pos)));
    }
    if input[pos] != b {
        return Err(LexError::invalid_char(
            input[pos] as char, 
            Location(pos, pos + 1),
        ));
    }

    Ok((b, pos + 1))
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

fn lex_number(input: &[u8], pos: usize) -> Result<(Token, usize), LexError> {
    use::std::str::from_utf8;

    let start = pos;
    let end = recognize_many(input, start, |b|b"1234567890".contains(&b));
    let n = from_utf8(&input[start..end])
        .unwrap()
        .parse()
        .unwrap();
    Ok((Token::number(n, Location(start, end)), end))
}

fn skip_spaces(input: &[u8], pos: usize) -> Result<((),usize), LexError> {
    let pos = recognize_many(input, pos, |b| b"\n\t".contains(&b));
    Ok(((), pos))
}

fn recognize_many(input: &[u8], mut pos: usize, mut f: impl FnMut(u8) -> bool) -> usize {
    while pos < input.len() && f(input[pos]){
        pos += 1;
    }
    pos
}

#[cfg(test)]
mod tests {
    use super::*;

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
