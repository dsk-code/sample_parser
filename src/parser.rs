use crate::ast::{Ast, Astkind, BinaryOperation, ParseError, UnaryOperation};
use crate::token::{Token, TokenKind};

use std::iter::Peekable;

pub fn parse(tokens: Vec<Token>) -> Result<Ast, ParseError> {
    // 入力をイテレーターにし、、Peekableにする
    let mut tokens = tokens.into_iter().peekable();
    // その後parse_exprをよんでエラー処理をする
    let ret = parse_expr(&mut tokens)?;
    match tokens.next() {
        Some(token) => Err(ParseError::RedundantExpression(token)),
        None => Ok(ret),
    }
}

pub fn parse_expr<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    parse_expr3(tokens)
}

pub fn parse_expr3<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    // parse_left_binopに渡す関数を定義する
    fn parse_expr3_op<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<BinaryOperation, ParseError>
    where
        Tokens: Iterator<Item = Token>,
    {
        let op = tokens
            .peek()
            // イテレータの終わり入力の終端なのでエラーを出す
            .ok_or(ParseError::Eof)
            // エラーを返すかもしれない値をつなげる
            .and_then(|tok| match tok.value {
                TokenKind::Plus => Ok(BinaryOperation::add(tok.loc.clone())),
                TokenKind::Minus => Ok(BinaryOperation::sub(tok.loc.clone())),
                _ => Err(ParseError::NotOperator(tok.clone())),
            })?;
        tokens.next();
        Ok(op)
    }
    parse_left_binop(tokens, parse_expr2, parse_expr3_op)
}

pub fn parse_expr2<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    fn parse_expr2_op<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<BinaryOperation, ParseError>
    where
        Tokens: Iterator<Item = Token>,
    {
        let op = tokens
            .peek()
            .ok_or(ParseError::Eof)
            .and_then(|tok| match tok.value {
                TokenKind::Asterisk => Ok(BinaryOperation::mult(tok.loc.clone())),
                TokenKind::Slash => Ok(BinaryOperation::div(tok.loc.clone())),
                _ => Err(ParseError::NotOperator(tok.clone())),
            })?;
        tokens.next();
        Ok(op)
    }
    parse_left_binop(tokens, parse_expr1, parse_expr2_op)
}

pub fn parse_left_binop<Tokens>(
    tokens: &mut Peekable<Tokens>,
    subexpr_paser: fn(&mut Peekable<Tokens>) -> Result<Ast, ParseError>,
    op_parser: fn(&mut Peekable<Tokens>) -> Result<BinaryOperation, ParseError>,
) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    let mut e = subexpr_paser(tokens)?;
    loop {
        match tokens.peek() {
            Some(_) => {
                let op = match op_parser(tokens) {
                    Ok(op) => op,
                    // ここでパースに失敗したのはこれ以上中置演算子がないという意味
                    Err(_) => break,
                };
                let r = subexpr_paser(tokens)?;
                let loc = e.loc.merge(&r.loc);
                e = Ast::binary_operation(op, e, r, loc)
            }
            _ => break,
        }
    }
    Ok(e)
}

// expr1
pub fn parse_expr1<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    match tokens.peek().map(|tok| tok.value) {
        Some(TokenKind::Plus) | Some(TokenKind::Minus) => {
            // ("+"|"-")
            let op = match tokens.next() {
                Some(Token {
                    value: TokenKind::Plus,
                    loc,
                }) => UnaryOperation::plus(loc),
                Some(Token {
                    value: TokenKind::Minus,
                    loc,
                }) => UnaryOperation::minus(loc),
                _ => unreachable!(),
            };
            // ,ATOM
            let e = parse_atom(tokens)?;
            let loc = op.loc.merge(&e.loc);
            Ok(Ast::unary_operation(op, e, loc))
        }
        // |ATOM
        _ => parse_atom(tokens),
    }
}
// ATOM
pub fn parse_atom<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    tokens
        .next()
        .ok_or(ParseError::Eof)
        .and_then(|tok| match tok.value {
            // UNUMBER
            TokenKind::Number(n) => Ok(Ast::new(Astkind::Number(n), tok.loc)),
            // | "(",EXPR3,")";
            TokenKind::Lparen => {
                let e = parse_expr(tokens)?;
                match tokens.next() {
                    Some(Token {
                        value: TokenKind::Rparen,
                        loc: _,
                    }) => Ok(e),
                    Some(t) => Err(ParseError::RedundantExpression(t)),
                    _ => Err(ParseError::UnclosedOpenParen(tok)),
                }
            }
            _ => Err(ParseError::NotExpression(tok)),
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Location;

    #[test]
    fn test_parser() {
        // 1 + 2 * 3 - -10
        let ast = parse(vec![
            Token::number(1, Location(0, 1)),
            Token::plus(Location(2, 3)),
            Token::number(2, Location(4, 5)),
            Token::asterisk(Location(6, 7)),
            Token::number(3, Location(8, 9)),
            Token::minus(Location(10, 11)),
            Token::minus(Location(12, 13)),
            Token::number(10, Location(13, 15)),
        ]);
        assert_eq!(
            ast,
            Ok(Ast::binary_operation(
                BinaryOperation::sub(Location(10, 11)),
                Ast::binary_operation(
                    BinaryOperation::add(Location(2, 3)),
                    Ast::number(1, Location(0, 1)),
                    Ast::binary_operation(
                        BinaryOperation::new(crate::ast::BinaryOperationKind::Mult, Location(6, 7)),
                        Ast::number(2, Location(4, 5)),
                        Ast::number(3, Location(8, 9)),
                        Location(4, 9)
                    ),
                    Location(0, 9),
                ),
                Ast::unary_operation(
                    UnaryOperation::minus(Location(12, 13)),
                    Ast::number(10, Location(13, 15)),
                    Location(12, 15)
                ),
                Location(0, 15)
            ))
        )
    }
}
// parse_expr3()は書き換えたので古い１
// pub fn parse_expr3<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
// where
//     Tokens: IntoIterator<IntoIter = Token>,
// {
//     // 最初にEXPR3("+"|"-")Expr2を試す
//     // まずEXPR3をパース
//     match parse_expr3(tokens) {
//         // 失敗したらparse_expr2にフォールバック(|EXP2の部分)
//         Err(_) => parse_expr2(tokens),
//         // 成功したら
//         Ok(e) => {
//             // peekで先読みして
//             match tokens.peek().map(|tok| tok.value) {
//                 // ("+"|"-")である事を確認する。| を使ってパターンマッチを複数並べられる
//                 Some(TokenKind::Plus) | Some(TokenKind::Minus) => {
//                     // ("+"|"-")であれば入力を消費してパースを始める
//                     let op = match tokens.next().unwrap() {
//                         // Tokenは型エイリアスだがパターンマッチにも使える
//                         Token {
//                             // パターンマッチはネスト可能
//                             value: Tokenkind::Plus,
//                             loc,
//                         } => BinOp::add(loc),
//                         Token {
//                             value: Tokenkind::Minus,
//                             loc,
//                         } => BinOp::sub(loc),
//                         _ => unreachable!(),
//                     };
//                     // EXPR2をパース
//                     let r = Parse_expr2(tokens)?;
//                     // 結果は加減
//                     let loc = e.loc.merge(&r.loc)?;
//                     Ok(Ast::binop(op, e, r, loc))
//                 }
//                 // それ以外はエラー。エラーの種で処理を分ける
//                 Some(_) => Err(ParseError::UnexpectedToken(tokens.next().unwrap())),
//                 None => Err(ParseError::Eof),
//             }
//         }
//     }
// }

// parse_expr3()は書き換えたので古い２
// fn parse_expr3<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
//     where
//         Tokens: Iterator<Item = Token>,
//     {
//         // EXPR2をパースする
//         let mut e = parse_expr2(tokens)?;
//         // EXPR3_Loop
//         loop {
//             match tokens.peek().map(|tok| tok.value) {
//                 // ("+"|"-")
//                 Some(TokenKind::Plus) | Some(TokenKind::Minus) => {
//                     let op = match tokens.next().unwrap() {
//                         Token {
//                             value: TokenKind::Plus,
//                             loc,
//                         } => BinaryOperation::add(loc),
//                         Token {
//                             value: TokenKind::Minus,
//                             loc,
//                         } => BinaryOperation::sub(loc),
//                         _ => unreachable!(),
//                     };
//                     // EXPR2
//                     let r = parse_expr2(tokens)?;
//                     // 位置情報やAST構築の処理
//                     let loc = e.loc.merge(&r.loc);
//                     e = Ast::binary_operation(op, e, r, loc)
//                     // 次のイテレーションはEXPR3_Loop
//                 }
//                 // ε
//                 _ => return Ok(e),
//             }
//         }
//     }

// parse_expr3同様に書き換えするのでこれは古い
// pub fn parse_expr2<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
//     where
//         Tokens: Iterator<Item = Token>,
//     {
//         let mut e = parse_expr1(tokens)?;
//         loop {
//             match tokens.peek().map(|tok| tok.value) {
//                 Some(TokenKind::Asterisk) | Some(TokenKind::Slash) => {
//                     let op = match tokens.next().unwrap() {
//                         Token {
//                             value: TokenKind::Asterisk,
//                             loc,
//                         } => BinaryOperation::mult(loc),
//                         Token {
//                             value: TokenKind::Slash,
//                             loc,
//                         } => BinaryOperation::div(loc),
//                         _ => unreachable!(),
//                     };
//                     let r = parse_expr2(tokens)?;
//                     let loc = e.loc.merge(&r.loc);
//                     e = Ast::binary_operation(op, e, r, loc)
//                 }
//                 _ => return Ok(e),
//             }
//         }
//     }

// このparse_expr2は書き換えたので古い
// pub fn parse_expr2<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
//     where
//         Tokens: Iterator<Item = Token>,
//     {
//         let mut e = parse_expr1(tokens)?;
//         loop {
//             match tokens.peek().map(|tok| tok.value) {
//                 Some(TokenKind::Asterisk) | Some(TokenKind::Slash) => {
//                     let op = match tokens.next().unwrap() {
//                         Token {
//                             value: TokenKind::Asterisk,
//                             loc,
//                         } => BinaryOperation::mult(loc),
//                         Token {
//                             value: TokenKind::Slash,
//                             loc,
//                         } => BinaryOperation::div(loc),
//                         _ => unreachable!(),
//                     };
//                     let r = parse_expr2(tokens)?;
//                     let loc = e.loc.merge(&r.loc);
//                     e = Ast::binary_operation(op, e, r, loc)
//                 }
//                 _ => return Ok(e),
//             }
//         }
//     }
