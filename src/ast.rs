use crate::token::{Location, Annotation, Token};

// ASTを表すデータ型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Astkind { 
    // 数値
    Number(u64),
    // 単項演算
    UnaryOperation {
        operation: UnaryOperation,
        expression: Box<Ast>,
    },
    // 二項演算
    BinaryOperation {
        operation: BinaryOperation,
        left: Box<Ast>,
        right: Box<Ast>,
    },
}

pub type Ast = Annotation<Astkind>;

// ヘルパーメソッドを定義しておく
impl Ast {
    pub fn number(n: u64, loc: Location) ->Self {
        // impl<T> Annotation<T> で実装したnewを呼ぶ
        Self::new(Astkind::Number(n), loc)
    }

    pub fn unary_operation(operation: UnaryOperation, expression: Ast, loc: Location) ->Self {
        Self::new(
            Astkind::UnaryOperation {
            operation,
            expression: Box::new(expression)
            },
            loc
        )
    }

    pub fn binary_operation(operation: BinaryOperation, left: Ast, right: Ast, loc: Location) -> Self {
        Self::new(
            Astkind::BinaryOperation {
            operation,
            left: Box::new(left),
            right: Box::new(right),
            },
            loc
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnaryOperationKind {
    // 正号
    Plus,
    // 負号
    Minus,
}

pub type UnaryOperation = Annotation<UnaryOperationKind>;

impl UnaryOperation {
    pub fn plus(loc: Location) -> Self {
        Self::new(UnaryOperationKind::Plus, loc)
    }

    pub fn minus(loc: Location) -> Self {
        Self::new(UnaryOperationKind::Minus, loc)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOperationKind {
    // 加算
    Add,
    // 減算
    Sub,
    // 乗算
    Mult,
    // 除算
    Div,
}

type BinaryOperation = Annotation<BinaryOperationKind>;

impl BinaryOperation {
    pub fn add(loc: Location) -> Self {
        Self::new(BinaryOperationKind::Add, loc)
    }

    pub fn sub(loc: Location) -> Self {
        Self::new(BinaryOperationKind::Sub, loc)
    }

    pub fn mult(loc: Location) -> Self {
        Self::new(BinaryOperationKind::Mult, loc)
    }

    pub fn div(loc: Location) -> Self {
        Self::new(BinaryOperationKind::Div, loc)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseError {
    // 予期しないトークンがきた
    UnexpectedToken(Token),
    // 式を期待していたのに式でないものがきた
    NotExpression(Token),
    // 演算子を期待していたのに演算子でないものがきた
    NotOperator(Token),
    // 括弧が閉じられていない
    UnclosedOpenParen(Token),
    // 式の解析が終わったのにまだトークンが残っている
    RedundantExpression(Token),
    // パースの途中で入力が終わった
    Eof,
}
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_ast_number() {
//         let test_number = 3;
//         let test_location = Location(0, 2);
//         let result = Ast::number(test_number, test_location);

//     }
// }