// fn print_parse_info(msg: &str) {
//     println!("[info][parse] {} ", msg);
// }

// fn print_parse_more1_info<T: std::fmt::Debug>(msg: &str, t: T) {
//     println!("[info][parse] {}, {:?}", msg, t);
// }

pub fn print_parse_more2_info<T: std::fmt::Debug, K: std::fmt::Debug>(_msg: &str, _t1: T, _t2: K) {
    // println!("[info][parse] {}, {:?}, {:?}", msg, t1, t2);
}