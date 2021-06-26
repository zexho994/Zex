use lexer::token;
use parse::parse;
use std::collections::HashMap;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::process;

fn main() {
    let mut var_map: HashMap<String, i32> = HashMap::new();
    loop {
        print!(">");
        stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        if input == "exit" {
            return;
        }
        let mut tokens = token::new_tokens(input.to_string());
        let num = parse::parse_tokens(&mut tokens, &mut var_map);
        println!("{}", num.unwrap());
    }
}
