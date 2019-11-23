use std::slice::Iter;

#[derive(Clone, Debug, PartialEq)]
enum Register {
    RDI,
    RSI,
    RAX,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Push(i32),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Ret,
}

pub fn code_gen(ops: Iter<Op>) -> String {
    use Op::*;
    let code = ops.fold("".to_string(), |sum, current| {
        let code = match current {
            Push(i) => format!("push {}",i),
            Add => format!("pop rdi\npop rax\nadd rax,rdi\npush rax\n"),
            Mul => format!("pop rdi\npop rax\nimul rax,rdi\npush rax\n"),
            Sub => format!("pop rdi\npop rax\nsub rax,rdi\npush rax\n"),
            Div => format!("pop rdi\npop rax\nidiv rax,rdi\npush rax\n"),
            _ => format!("")
        };
        format!("{}{}\n", sum, code)
    });
    let prefix = ".intel_syntax noprefix\n.global _main\n_main:\n";
    format!("{}{}pop rax\nret",prefix,code)
}
