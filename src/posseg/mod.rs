extern crate regex;


mod prob_start;
mod prob_trans;
mod prob_emit;
mod char_state_tap;

use regex::Regex;

lazy_static! {
    static ref RE_HAN_DETAIL: Regex = Regex::new(r"([\x{4E00}-\x{9FD5}]+)").unwrap();
    static ref RE_SKIP_DETAIL: Regex = Regex::new(r"([\.0-9]+|[a-zA-Z0-9]+)").unwrap();
    static ref RE_HAN_INTERNAL: Regex = Regex::new(r"([\x{4E00}-\x{9FD5}a-zA-Z0-9+#&\._]+)").unwrap();
    static ref RE_SKIP_INTERNAL: Regex = Regex::new(r"(\r\n|\s)").unwrap();
    static ref RE_ENG: Regex = Regex::new(r"[a-zA-Z0-9]+").unwrap();
    static ref RE_NUM: Regex = Regex::new(r"[\.0-9]+").unwrap();
    static ref RE_ENG1: Regex = Regex::new(r"^[a-zA-Z0-9]$").unwrap();
}
