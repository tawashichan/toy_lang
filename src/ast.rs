use std::iter::Peekable;
use std::slice::Iter;

use crate::lexer::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum Exp {
    MulSub(MulSub),
    PlusMinus(PlusMinus, Box<Exp>, Box<Exp>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MulSub {
    Mul(Box<MulSub>, Box<MulSub>),
    Sub(Box<MulSub>, Box<MulSub>),
    Primary(Box<Primary>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Primary {
    Int(i32),
    Exp(Exp),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PlusMinus {
    Plus,
    Minus,
}

pub fn parse_exp(tokens: &mut Peekable<Iter<Token>>) -> Exp {
    let primary = parse_primary(tokens);
    let mul_sub = parse_mul_sub(tokens, MulSub::Primary(Box::new(primary)));
    parse_exp_sub(tokens, Exp::MulSub(mul_sub))
}

fn parse_exp_sub(tokens: &mut Peekable<Iter<Token>>, exp: Exp) -> Exp {
    while let Some(token) = tokens.peek() {
        match *token {
            Token::Plus => {
                tokens.next();
                let exp2 = Exp::MulSub(parse_mul_sub_main(tokens));
                let exp = Exp::PlusMinus(PlusMinus::Plus, Box::new(exp), Box::new(exp2));
                return parse_exp_sub(tokens, exp);
            }
            Token::Minus => {
                tokens.next();
                let exp2 = Exp::MulSub(parse_mul_sub_main(tokens));
                let exp = Exp::PlusMinus(PlusMinus::Minus, Box::new(exp), Box::new(exp2));
                return parse_exp_sub(tokens, exp);
            }
            _ => return exp,
        }
    }
    return exp;
}

fn parse_mul_sub_main(tokens: &mut Peekable<Iter<Token>>) -> MulSub {
    let primary = parse_primary(tokens);
    parse_mul_sub(tokens, MulSub::Primary(Box::new(primary)))
}

fn parse_mul_sub(tokens: &mut Peekable<Iter<Token>>, mul_sub: MulSub) -> MulSub {
    while let Some(token) = tokens.peek() {
        match *token {
            Token::Mul => {
                tokens.next();
                let primary2 = MulSub::Primary(Box::new(parse_primary(tokens)));
                let mul_sub = MulSub::Mul(Box::new(mul_sub), Box::new(primary2));
                return parse_mul_sub(tokens, mul_sub);
            }
            Token::Div => {
                tokens.next();
                let primary2 = MulSub::Primary(Box::new(parse_primary(tokens)));
                let mul_sub = MulSub::Sub(Box::new(mul_sub), Box::new(primary2));
                return parse_mul_sub(tokens, mul_sub);
            }
            _ => return mul_sub,
        }
    }
    return mul_sub;
}

fn parse_primary(tokens: &mut Peekable<Iter<Token>>) -> Primary {
    while let Some(token) = tokens.peek() {
        match *token {
            Token::Int(i) => {
                tokens.next();
                return Primary::Int(*i);
            }
            Token::LPar => {
                tokens.next();
                let exp = parse_exp(tokens);
                let tok = tokens.peek();
                match tok {
                    Some(Token::RPar) => {
                        tokens.next();
                        return Primary::Exp(exp);
                    }
                    _ => panic!("{:?}", tokens),
                }
            }
            _ => panic!("{:?}", tokens),
        }
    }
    panic!("{:?}", tokens)
}
