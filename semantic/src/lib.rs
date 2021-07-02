#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let str = "int i = 1 + 1;";
        let mut tokens = lexer::lexing(str.to_string());
        let ast = parse::parsing(&mut tokens);
        println!("ast is {:?}", ast);
    }
}
