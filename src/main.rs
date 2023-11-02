use token::{Annotation, Location};

use crate::token::{LexError, LexErrorKind, Token, TokenKind};

pub mod token;

pub mod lexer;

fn main() {
    let location = Location(2, 6);
    let location2 = Location(1, 5);
    let location3 = location.merge(&location2);
    println!("location = {:?}", location);
    println!("location2 = {:?}", location2);
    println!("location3 = {:?}", location3);

    let annotation = Annotation::new("はい", location);
    println!("annotation = {:?}", annotation);
    println!("annotation.value = {:?}", annotation.value);

    let number = Token::number(5, location2);
    match number.value {
        TokenKind::Number(x) => println!("number = {:?}", x),
        _ => (),
    }

    let lexerror = LexError::invalid_char('e', location3);
    match lexerror.value {
        LexErrorKind::InvalidChar(x) => println!("lexerror = {:?}", x),
        _ => (),
    }
    // println!("number = {:?}", number.value);
}
