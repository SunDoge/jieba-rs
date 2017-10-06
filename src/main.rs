extern crate clap;
extern crate jiebars;

// use std::collections::HashMap;
use clap::{App, Arg, SubCommand};

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
    // let mut tk = jiebars::Tokenizer::new(None);
    // let sentence = "小明硕士";
    // println!("{:?}", tk.cut(&sentence, false, true));
    // println!("{:?}", jiebars::cut(&sentence, false, true));
    // println!("{:?}", tk.cut("如果放到post中将出错。", false, false));
    // println!("{:?}", tk.cut_for_search("小明硕士毕业于中国科学院计算所，后在日本京都大学深造", true));
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

    let matches = App::new("jiebars")
        .version(jiebars::VERSION)
        .author("SunDoge")
        .about("Jieba Command line interface")
        .arg(
            Arg::with_name("delimiter")
                .short("d")
                .long("delimiter")
                .value_name("DELIM")
                .help(
                    "use DELIM instead of ' / ' for word delimiter; or a space if it is used without DELIM",
                ),
        )
        .arg(
            Arg::with_name("POS")
                .short("p")
                .long("pos")
                .value_name("DELIM")
                .help(
                    "enable POS tagging; if DELIM is specified, use DELIM instead of '_' for POS delimiter",
                ),
        )
        .arg(
            Arg::with_name("DICT")
                .short("D")
                .long("dict")
                .value_name("DICT")
                .help(
                    "use DICT as dictionary",
                ),
        )
        .arg(
            Arg::with_name("USER_DICT")
                .short("u")
                .long("user-dict")
                .value_name("USER_DICT")
                .help(
                    "use USER_DICT together with the default dictionary or DICT (if specified)",
                ),
        )
        .arg(
            Arg::with_name("cut_all")
                .short("a")
                .long("cut-all")
                .help(
                    "full pattern cutting (ignored with POS tagging)",
                ),
        )
        .arg(
            Arg::with_name("no_hmm")
                .short("n")
                .long("no-hmm")
                .help(
                    "don't use the Hidden Markov Model",
                ),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help(
                    "don't print loading messages to stderr",
                ),
        )
        .arg(Arg::with_name("FILENAME").help("input file").required(true).index(1))
        .get_matches();


    // Gets a value for config if supplied by user, or defaults to "default.conf"
    // let config = matches.value_of("config").unwrap_or("default.conf");
    // println!("Value for config: {}", config);

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    println!("Using input file: {}", matches.value_of("FILENAME").unwrap());

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    // match matches.occurrences_of("v") {
    //     0 => println!("No verbose info"),
    //     1 => println!("Some verbose info"),
    //     2 => println!("Tons of verbose info"),
    //     3 | _ => println!("Don't be crazy"),
    // }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    // if let Some(matches) = matches.subcommand_matches("test") {
    //     if matches.is_present("debug") {
    //         println!("Printing debug info...");
    //     } else {
    //         println!("Printing normally...");
    //     }
    // }
}
