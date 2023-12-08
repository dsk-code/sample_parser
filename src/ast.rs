use crate::token::{Annotation, Location, Token};

// 抽象構文木(AST)を表すデータ型
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
    pub fn number(n: u64, loc: Location) -> Self {
        // impl<T> Annotation<T> で実装したnewを呼ぶ
        Self::new(Astkind::Number(n), loc)
    }

    pub fn unary_operation(operation: UnaryOperation, expression: Ast, loc: Location) -> Self {
        Self::new(
            Astkind::UnaryOperation {
                operation,
                expression: Box::new(expression),
            },
            loc,
        )
    }

    pub fn binary_operation(
        operation: BinaryOperation,
        left: Ast,
        right: Ast,
        loc: Location,
    ) -> Self {
        Self::new(
            Astkind::BinaryOperation {
                operation,
                left: Box::new(left),
                right: Box::new(right),
            },
            loc,
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

pub type BinaryOperation = Annotation<BinaryOperationKind>;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_number() {
        let test_number = 3;
        let test_location = Location(0, 2);
        let expected_result = Annotation {
            value: Astkind::Number(3),
            loc: Location(0, 2),
        };
        let result = Ast::number(test_number, test_location);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_ast_unary_operation() {
        let test_unary_operation = UnaryOperation::minus(Location(2, 4));
        let test_ast = Annotation {
            value: Astkind::Number(3),
            loc: Location(4, 5),
        };
        let test_location = Location(0, 2);
        let expect_result = Annotation {
            value: Astkind::UnaryOperation {
                operation: Annotation {
                    value: UnaryOperationKind::Minus,
                    loc: Location(2, 4),
                },
                expression: Box::new(Annotation {
                    value: Astkind::Number(3),
                    loc: Location(4, 5),
                }),
            },
            loc: Location(0, 2),
        };
        let result = Ast::unary_operation(test_unary_operation, test_ast, test_location);
        assert_eq!(expect_result, result);
    }

    #[test]
    fn test_ast_binary_operation() {
        let test_binary_operation = BinaryOperation::add(Location(0, 2));
        let test_binary_operation_left = Ast::number(3, Location(4, 5));
        let test_binary_operation_right = Ast::unary_operation(
            UnaryOperation::minus(Location(6, 7)),
            Ast::number(8, Location(9, 10)),
            Location(11, 12),
        );
        let test_location = Location(13, 14);
        let expected_result = Annotation {
            value: Astkind::BinaryOperation {
                operation: Annotation {
                    value: BinaryOperationKind::Add,
                    loc: Location(0, 2),
                },
                left: Box::new(Annotation {
                    value: Astkind::Number(3),
                    loc: Location(4, 5),
                }),
                right: Box::new(Annotation {
                    value: Astkind::UnaryOperation {
                        operation: Annotation {
                            value: UnaryOperationKind::Minus,
                            loc: Location(6, 7),
                        },
                        expression: Box::new(Annotation {
                            value: Astkind::Number(8),
                            loc: Location(9, 10),
                        }),
                    },
                    loc: Location(11, 12),
                }),
            },
            loc: Location(13, 14),
        };
        let result = Ast::binary_operation(
            test_binary_operation,
            test_binary_operation_left,
            test_binary_operation_right,
            test_location,
        );
        assert_eq!(expected_result, result);
    }
    #[test]
    fn test_unary_operation_plus() {
        let test_location = Location(0, 2);
        let expect_result = Annotation {
            value: UnaryOperationKind::Plus,
            loc: Location(0, 2),
        };
        let result = UnaryOperation::plus(test_location);
        assert_eq!(expect_result, result);
    }

    #[test]
    fn test_unary_operation_minus() {
        let test_location = Location(0, 2);
        let expect_result = Annotation {
            value: UnaryOperationKind::Minus,
            loc: Location(0, 2),
        };
        let result = UnaryOperation::minus(test_location);
        assert_eq!(expect_result, result);
    }

    #[test]
    fn test_binary_operation_add() {
        let test_location = Location(0, 2);
        let expect_result = Annotation {
            value: BinaryOperationKind::Add,
            loc: Location(0, 2),
        };
        let result = BinaryOperation::add(test_location);
        assert_eq!(expect_result, result);
    }

    #[test]
    fn test_binary_operation_sub() {
        let test_location = Location(0, 2);
        let expect_result = Annotation {
            value: BinaryOperationKind::Sub,
            loc: Location(0, 2),
        };
        let result = BinaryOperation::sub(test_location);
        assert_eq!(expect_result, result);
    }

    #[test]
    fn test_binary_operation_mult() {
        let test_location = Location(0, 2);
        let expect_result = Annotation {
            value: BinaryOperationKind::Mult,
            loc: Location(0, 2),
        };
        let result = BinaryOperation::mult(test_location);
        assert_eq!(expect_result, result);
    }

    #[test]
    fn test_binary_operation_div() {
        let test_location = Location(0, 2);
        let expect_result = Annotation {
            value: BinaryOperationKind::Div,
            loc: Location(0, 2),
        };
        let result = BinaryOperation::div(test_location);
        assert_eq!(expect_result, result);
    }
}
