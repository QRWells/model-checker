use model_checker::{
    ctl::to_normal_form,
    parser::{ctl_parser::parse_ctl, ltl_parser::parse_ltl},
};

fn main() {
    println!("Hello, world!");
    let res = parse_ctl("A(a U (b \\/ c)))");
    if let Ok(res) = res {
        println!("{}", res);
        let res = to_normal_form(res);
        println!("{}", res.get_str());
    }
    let res = parse_ltl("G ( a U b )");
    if let Ok(res) = res {
        println!("{}", res);
    }
}
