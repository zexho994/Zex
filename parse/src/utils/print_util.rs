pub fn print_parse_info(_msg: &str) {
    // println!("[info][parse] {} ", msg);
}

pub fn print_parse_more1_info<T: std::fmt::Debug>(_msg: &str, _t: T) {
    // println!("[info][parse] {}, {:?}", msg, t);
}

pub fn print_parse_more2_info<T: std::fmt::Debug, K: std::fmt::Debug>(_msg: &str, _t1: T, _t2: K) {
    println!("[info][parse] {}, {:?}, {:?}", _msg, _t1, _t2);
}
