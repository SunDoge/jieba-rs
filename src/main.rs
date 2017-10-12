extern crate clap;
extern crate jiebars;

// use std::collections::HashMap;
use clap::{App, Arg, SubCommand};

fn main() {

    // test();
    test_userdict();
    
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
    println!("{}", jiebars::cut("如果放到post中将出错。", false, false).join("/"));

    println!("{}", jiebars::cut("「台中」正确应该不会被切开", false, false).join("/"));

    println!("{}", jiebars::suggest_freq(&vec!["台中"], true));

    println!("{}", jiebars::cut("「台中」正确应该不会被切开", false, false).join("/"));

    println!("{}", "=".repeat(40));
    println!("3. 关键词提取");
    println!("{}", "-".repeat(40));
    println!(" TF-IDF");
    println!("{}", "-".repeat(40));
    


    println!("{}", "-".repeat(40));
    println!(" TextRank");
    println!("{}", "-".repeat(40));




    println!("{}", "=".repeat(40));
    println!("4. 词性标注");
    println!("{}", "-".repeat(40));



    println!("{}", "=".repeat(40));
    println!("6. Tokenize: 返回词语在原文的起止位置");
    println!("{}", "-".repeat(40));
    println!("默认模式");
    println!("{}", "-".repeat(40));

    let result = jiebars::tokenize("永和服装饰品有限公司", jiebars::Mode::Default, true);
    for tk in result {
        println!("word {}\t\t start: {} \t\t end:{}", tk.0, tk.1, tk.2);
    }

    println!("{}", "-".repeat(40));
    println!("搜索模式");
    println!("{}", "-".repeat(40));
    let result = jiebars::tokenize("永和服装饰品有限公司", jiebars::Mode::Search, true);
    for tk in result {
        println!("word {}\t\t start: {} \t\t end:{}", tk.0, tk.1, tk.2);
    }
    
}

fn test_userdict() {
    jiebars::load_user_dict("tests/userdict.txt");

    jiebars::add_word("石墨烯", &None, &None);
    jiebars::add_word("凱特琳", &None, &None);
    jiebars::del_word("自定义词");

    // let test_sent = "李小福是创新办主任也是云计算方面的专家; 什么是八一双鹿\n例如我输入一个带“韩玉赏鉴”的标题，在自定义词库中也增加了此词为N类\n「台中」正確應該不會被切開。mac上可分出「石墨烯」；此時又可以分出來凱特琳了。";
    let test_sent = "mac上可分出「石墨烯」；";

    let words = jiebars::cut(test_sent, false, true);

    println!("{}", words.join("/"));
}
