extern crate regex;
extern crate serde_pickle;


// mod prob_start;
// mod prob_trans;
// mod prob_emit;
// mod char_state_tap;

use regex::Regex;
use std::collections::HashMap as Map;
use super::compact::{get_module_res};
use std::fs::File;
use std::io::prelude::*;

pub type CharStateTap = Map<char, Vec<(char, String)>>;
pub type ProbStart = Map<(char, String), f64>;
pub type ProbTrans = Map<(char, String), Map<(char, String), f64>>;
pub type ProbEmit = Map<(char, String), Map<char, f64>>;

const PROB_START_P: &'static str = "prob_start.p";
const PROB_TRANS_P: &'static str = "prob_trans.p";
const PROB_EMIT_P: &'static str = "prob_emit.p";
const CHAR_STATE_TAB_P: &'static str = "char_state_tab.p";


lazy_static! {
    static ref RE_HAN_DETAIL: Regex = Regex::new(r"([\x{4E00}-\x{9FD5}]+)").unwrap();
    static ref RE_SKIP_DETAIL: Regex = Regex::new(r"([\.0-9]+|[a-zA-Z0-9]+)").unwrap();
    static ref RE_HAN_INTERNAL: Regex = Regex::new(r"([\x{4E00}-\x{9FD5}a-zA-Z0-9+#&\._]+)").unwrap();
    static ref RE_SKIP_INTERNAL: Regex = Regex::new(r"(\r\n|\s)").unwrap();
    static ref RE_ENG: Regex = Regex::new(r"[a-zA-Z0-9]+").unwrap();
    static ref RE_NUM: Regex = Regex::new(r"[\.0-9]+").unwrap();
    static ref RE_ENG1: Regex = Regex::new(r"^[a-zA-Z0-9]$").unwrap();

    static ref START_P: ProbStart = serde_pickle::from_reader(load_model(PROB_START_P)).unwrap();
    static ref TRANS_P: ProbTrans = serde_pickle::from_reader(load_model(PROB_TRANS_P)).unwrap();
    static ref EMIT_P: ProbEmit = serde_pickle::from_reader(load_model(PROB_EMIT_P)).unwrap();
    static ref STATE: CharStateTap = serde_pickle::from_reader(load_model(CHAR_STATE_TAB_P)).unwrap();
}

fn load_model(filename: &str) -> File {
    let mut contents = String::new();
    let res = get_module_res(&vec!["posseg", filename]);
    // println!("{}", &res);
    let f = File::open(&res).expect("file not found");
    f
}