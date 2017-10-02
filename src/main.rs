extern crate jiebars;

// use std::collections::HashMap;

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
    let mut tk = jiebars::Tokenizer::new(None);
    let sentence = "小明硕士";
    println!("{:?}", tk.cut(&sentence, false, true));
    println!("{:?}", tk.cut("如果放到post中将出错。", false, false));
    // println!("{:?}", *jiebars::finalseg::prob_start::P);
    // println!("{:?}", *jiebars::finalseg::prob_start::P);
    // println!("{:?}", tk.get_dag(&sentence));
    // let contents = tk.get_dict_file().unwrap();
    // let (freq, total) = tk.gen_pfdict(&contents);
    // println!("{:?}: {}", freq, total);
    // jiebars::enable_parallel(0);
    // let n = 5;
    // for i in (-1..n - 1).rev() {
    //     println!("{}", &i);
    // }
}