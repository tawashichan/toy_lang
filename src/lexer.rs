use std::slice::Iter;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Int(i32),
    Plus,
    Minus,
    Mul,
    Sub,
    LPar,
    RPar,
}

fn parse_num(first: char,chars: &mut Iter<char>) -> i32 {
    let mut buf: Vec<char> = vec![];
    buf.push(first);
    while let Some(ch) = chars.next() {
        if ch.is_numeric() {
            buf.push(*ch)
        } else {
            break;
        }
    }
    let num_str = buf.iter().map(|c| c.to_string()).collect::<String>();
    num_str.parse::<i32>().unwrap()
}

pub fn parse_token(chars: &[char]) -> Vec<Token> {
    let buf = &mut vec![];
    let tokens = parse_token_sub(chars, buf);
    tokens.to_vec()
}

fn parse_token_sub<'a>(chars: &[char], tokens: &'a mut Vec<Token>) -> &'a [Token] {
    use Token::*;
    let mut iter = chars.iter();
    while let Some(ch) = iter.next() {
        match ch {
            ' ' | '\t' | '\n' => continue,
            '+' => tokens.push(Plus),
            '-' => tokens.push(Minus),
            '*' => tokens.push(Mul),
            '(' => tokens.push(LPar),
            ')' => tokens.push(RPar),
            _ => {
                let num = parse_num(*ch,&mut iter);
                tokens.push(Int(num))
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
