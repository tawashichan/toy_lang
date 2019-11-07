use crate::ast::{Exp, MulSub, PlusMinus, Primary};

use crate::stack_machine::*;

pub fn code_gen<'a>(exp: Exp) -> Vec<Op> {
    let mut op = vec![];
    exp_code_gen(exp, &mut op).to_vec()
}

fn exp_code_gen(exp: Exp, ops: &mut Vec<Op>) -> &mut Vec<Op> {
    match exp {
        Exp::PlusMinus(op, exp1, exp2) => match op {
            PlusMinus::Plus => {
                exp_code_gen(*exp2, exp_code_gen(*exp1, ops));
                ops.push(Op::Add);
                ops
            }
            PlusMinus::Minus => {
                exp_code_gen(*exp2, exp_code_gen(*exp1, ops));
                ops.push(Op::Sub);
                ops
            }
        },
        Exp::MulSub(mul_sub) => mul_sub_code_gen(mul_sub, ops)
    }
}

fn mul_sub_code_gen(mul_sub: MulSub, ops: &mut Vec<Op>) -> &mut Vec<Op> {
    match mul_sub {
        MulSub::Mul(p1, p2) => {
            mul_sub_code_gen(*p2, mul_sub_code_gen(*p1, ops));
            ops.push(Op::Mul);
            ops
        }
        MulSub::Sub(p1, p2) => {
            mul_sub_code_gen(*p2, mul_sub_code_gen(*p1, ops));
            ops.push(Op::Sub);
            ops
        }
        MulSub::Primary(p) => primary_code_gen((*p).clone(), ops),
    }
}

fn primary_code_gen(primary: Primary, ops: &mut Vec<Op>) -> &mut Vec<Op> {
    match primary {
        Primary::Int(i) => {
            ops.push(Op::Push(i));
            ops
        }
        Primary::Exp(exp) => exp_code_gen(exp, ops),
    }
}
