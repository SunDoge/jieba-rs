extern crate jiebars;

use std::collections::HashMap;

fn main() {
    // let mut p = HashMap::new();
    // p.insert('\u{4e00}', vec![('B', "'nr'")]);
    // p.insert('\u{4e01}', vec![('B', "'nr'"), ('B', "nr")]);
    // print!("{:?}", p);
    // jiebars::posseg::char_state_tap::P(&mut p)
    let p = jiebars::posseg::char_state_tap::p();
    println!("{:?}", p);
}