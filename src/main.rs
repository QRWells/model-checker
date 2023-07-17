use model_checker::parser::{ctl_parser::parse_ctl, ltl_parser::parse_ltl};

fn main() {
    println!("Hello, world!");
    let res = parse_ctl("EX ( a U b )");
    println!("{:?}", res);
    let res = parse_ltl("G ( a U b )");
    println!("{:?}", res);
}
