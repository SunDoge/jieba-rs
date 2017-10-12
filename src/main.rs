extern crate clap;
extern crate jiebars;

// use std::collections::HashMap;
use clap::{App, Arg, SubCommand};

fn main() {

    test();
    
    // let matches = App::new("jiebars")
    //     .version(jiebars::VERSION)
    //     .author(jiebars::AUTHORS)
    //     .about("Jieba Command line interface")
    //     .arg(
    //         Arg::with_name("delimiter")
    //             .short("d")
    //             .long("delimiter")
    //             .value_name("DELIM")
    //             .help(
    //                 "use DELIM instead of ' / ' for word delimiter; or a space if it is used without DELIM",
    //             ),
    //     )
    //     .arg(
    //         Arg::with_name("POS")
    //             .short("p")
    //             .long("pos")
    //             .value_name("DELIM")
    //             .help(
    //                 "enable POS tagging; if DELIM is specified, use DELIM instead of '_' for POS delimiter",
    //             ),
    //     )
    //     .arg(
    //         Arg::with_name("DICT")
    //             .short("D")
    //             .long("dict")
    //             .value_name("DICT")
    //             .help(
    //                 "use DICT as dictionary",
    //             ),
    //     )
    //     .arg(
    //         Arg::with_name("USER_DICT")
    //             .short("u")
    //             .long("user-dict")
    //             .value_name("USER_DICT")
    //             .help(
    //                 "use USER_DICT together with the default dictionary or DICT (if specified)",
    //             ),
    //     )
    //     .arg(
    //         Arg::with_name("cut_all")
    //             .short("a")
    //             .long("cut-all")
    //             .help(
    //                 "full pattern cutting (ignored with POS tagging)",
    //             ),
    //     )
    //     .arg(
    //         Arg::with_name("no_hmm")
    //             .short("n")
    //             .long("no-hmm")
    //             .help(
    //                 "don't use the Hidden Markov Model",
    //             ),
    //     )
    //     .arg(
    //         Arg::with_name("quiet")
    //             .short("q")
    //             .long("quiet")
    //             .help(
    //                 "don't print loading messages to stderr",
    //             ),
    //     )
    //     .arg(Arg::with_name("FILENAME").help("input file").required(true).index(1))
    //     .get_matches();


    // // Gets a value for config if supplied by user, or defaults to "default.conf"
    // // let config = matches.value_of("config").unwrap_or("default.conf");
    // // println!("Value for config: {}", config);

    // // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // // required we could have used an 'if let' to conditionally get the value)
    // println!("Using input file: {}", matches.value_of("FILENAME").unwrap());

    // // Vary the output based on how many times the user used the "verbose" flag
    // // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    // // match matches.occurrences_of("v") {
    // //     0 => println!("No verbose info"),
    // //     1 => println!("Some verbose info"),
    // //     2 => println!("Tons of verbose info"),
    // //     3 | _ => println!("Don't be crazy"),
    // // }

    // // You can handle information about subcommands by requesting their matches by name
    // // (as below), requesting just the name used, or both at the same time
    // // if let Some(matches) = matches.subcommand_matches("test") {
    // //     if matches.is_present("debug") {
    // //         println!("Printing debug info...");
    // //     } else {
    // //         println!("Printing normally...");
    // //     }
    // // }
}

fn test() {
    println!("{}", "=".repeat(40));
    println!("1. 分词");
    println!("{}", "-".repeat(40));

    let seg_list = jiebars::cut("我来到北京清华大学", true, true);
    println!("Full Mode: {}", &seg_list.join("/ "));

    let seg_list = jiebars::cut("我来到北京清华大学", false, true);
    println!("Default Mode: {}", &seg_list.join("/ "));

    let seg_list = jiebars::cut("他来到了网易杭研大厦", false, true);
    println!("{}", &seg_list.join(", "));

    let seg_list = jiebars::cut_for_search("小明硕士毕业于中国科学院计算所，后在日本京都大学深造", true);
    println!("{}", &seg_list.join(", "));


    println!("{}", "=".repeat(40));
    println!("2. 添加自定义词典/调整词典");
    println!("{}", "-".repeat(40));

    println!("{}", jiebars::cut("如果放到post中将出错。", false, false).join("/"));

    println!("{}", jiebars::suggest_freq(&vec!["中", "将"], true));
    // 494
}
