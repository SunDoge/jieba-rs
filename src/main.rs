extern crate jiebars;

use std::collections::HashMap;

fn main() {
    // let mut p = HashMap::new();
    // p.insert('\u{4E00}', vec![('B', "'nr'")]);
    // p.insert('\u{4E01}', vec![('B', "'nr'"), ('B', "nr")]);
    // print!("{:?}", p);
    // jiebars::posseg::char_state_tap::P(&mut p)
    // let p = jiebars::posseg::char_state_tap::p();
    // println!("{:?}", p);

    // let s = "这是一个测试";
    // println!("words: {}", s);
    // println!("len = {}", s.chars().nth(0).unwrap() );
    // println!("s[0] = {}", s.chars().nth(0));
    // use std::env;
    // let mut path = env::current_dir().unwrap();
    // path.push("src");
    // path.push("dict.txt");
    // let path = "dick";
    // let path = jiebars::get_abs_path(path);
    // println!("{}", jiebars::get_abs_path(path));
    let tk = jiebars::Tokenizer::new(None);
    let sentence = "我来到北京清华大学";
    println!("{:?}", tk.cut(&sentence, true, true));

}