use std::iter::Peekable;
use std::slice::Iter;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Int(i32),
    Plus,
    Minus,
    Mul,
    Div,
    LPar,
    RPar,
    Let,
    Equal,
    Assign,
    Fun,
    Colon,
    Str(String),
}

fn parse_num(chars: &mut Peekable<Iter<char>>) -> i32 {
    let mut buf: Vec<char> = vec![];
    while let Some(ch) = chars.peek() {
        if ch.is_numeric() {
            buf.push(**ch);
            chars.next();
        } else {
            break;
        }
    }
    let num_str = buf.iter().map(|c| c.to_string()).collect::<String>();
    num_str.parse::<i32>().unwrap()
}

fn parse_string(chars: &mut Peekable<Iter<char>>) -> Token {
    use Token::*;
    let mut buf: Vec<char> = vec![];
    while let Some(ch) = chars.peek() {
        match ch {
            ' ' => break,
            ch => {
                buf.push(**ch);
                chars.next();
            }
        }
    }
    let string: String = buf.into_iter().collect();
    match string.as_str() {
        "let" => Let,
        "fun" => Fun,
        _ => Str(string),
    }
}

pub fn parse_token(chars: &mut [char]) -> Vec<Token> {
    let buf = &mut vec![];
    let tokens = parse_token_sub(&mut chars.iter().peekable(), buf);
    tokens.to_vec()
}

fn push_and_consume<'a>(
    chars: &mut Peekable<Iter<char>>,
    tokens: &'a mut Vec<Token>,
    token: Token,
) {
    chars.next();
    tokens.push(token);
}

fn parse_token_sub<'a>(
    chars: &mut Peekable<Iter<char>>,
    tokens: &'a mut Vec<Token>,
) -> &'a [Token] {
    use Token::*;
    while let Some(ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\n' => {
                chars.next();
                continue;
            }
            '+' => push_and_consume(chars, tokens, Plus),
            '-' => push_and_consume(chars, tokens, Minus),
            '*' => push_and_consume(chars, tokens, Mul),
            '/' => push_and_consume(chars, tokens, Div),
            '(' => push_and_consume(chars, tokens, LPar),
            ')' => push_and_consume(chars, tokens, RPar),
            '=' => push_and_consume(chars, tokens, Assign),
            ch if ch.is_numeric() => {
                let num = parse_num(chars);
                tokens.push(Int(num))
            }
            _ => {
                tokens.push(parse_string(chars))
            }
        }
    }
    tokens
}

#[test]
fn parse_add() {
    let s = "1 + 1";
    let chars: Vec<char> = s.chars().collect();
    let tokens = parse_token(&chars);
    assert_eq!(tokens, vec![Token::Int(1), Token::Plus, Token::Int(1)]);
}
