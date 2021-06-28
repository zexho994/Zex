use clap::App;
use clap::Arg;
use lexer::token;
use parse::parse;
use std::collections::HashMap;
use std::fs::File;
use std::io::stdin;
use std::io::stdout;
use std::io::Read;
use std::io::Write;
use std::path::Path;

fn main() {
    let matches = App::new("Zex Program")
        .version("1.0")
        .author("zouzhihao@gmail.com")
        .about("github url: https://github.com/zexho994/Zex")
        .arg(
            Arg::with_name("mode")
                .short("m")
                .long("mode")
                .value_name("input")
                .help("使用什么模式启动程序")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .value_name("path")
                .help("zex's file read path")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("file name")
                .help("the file name of zex from <path>")
                .takes_value(true),
        )
        .get_matches();
    let mode = matches.value_of("mode").unwrap_or("input");
    let path = matches
        .value_of("path")
        .unwrap_or("/Users/zexho/Github/Zex/sample");
    let file = matches.value_of("file").unwrap_or("");
    if mode == "input" {
        input_mode();
    } else if mode == "file" {
        file_mode(path, file);
    }
}

fn input_mode() {
    println!("=> 手动输入执行语句，以分号；结束.");
    let mut var_map: HashMap<String, i32> = HashMap::new();
    loop {
        print!(">");
        stdout().flush().expect("flush error!");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        if input == "exit" {
            break;
        }
        let mut tokens = token::new_tokens(input.to_string());
        let num = parse::parse_tokens(&mut tokens, &mut var_map);
        println!("{}", num.unwrap());
    }
}

fn file_mode(p: &str, n: &str) {
    let mut full_path = String::from(p);
    full_path.push_str("/");
    full_path.push_str(n);
    let path = Path::new(full_path.as_str());
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {},err = {}", full_path, why),
        Ok(file) => file,
    };
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(err) => panic!("couldn't read {},err {}", full_path, err),
        Ok(_) => println!("{}", content),
    }
}
