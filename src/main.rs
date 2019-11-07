mod ast;
mod lexer;
mod code_gen;
mod stack_machine;

fn main() {
    let mut source = "1 + 2 * 6 - 1".chars().collect::<Vec<char>>();
    let tokens = lexer::parse_token(&mut source); 
    let exp = ast::parse_exp(&mut tokens.iter().peekable());
    let ops = code_gen::code_gen(exp);
    let code = stack_machine::code_gen(ops.iter());
    println!("{}",code);
}
