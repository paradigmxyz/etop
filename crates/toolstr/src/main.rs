use toolstr::format_num;

fn main() {
    // let as_str = format_num(".5", 6518);
    // let as_str = format_num(".2s", 0.012345);
    let as_str = format_num(".0%", 0.123);
    println!("{}", as_str)
}
