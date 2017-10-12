extern crate regex;
extern crate parking_lot;


mod prob_start;
mod prob_trans;
mod prob_emit;


use regex::Regex;
use std::collections::HashMap as Map;
use std::collections::HashSet as Set;
use parking_lot::Mutex;
use super::compact::{char_slice, SplitCaptures, SplitState};


pub type ProbEmit = Map<char, Map<char, f64>>;
pub type ProbStart = Map<char, f64>;
pub type ProbTrans = Map<char, Map<char, f64>>;

const MIN_FLOAT: f64 = -3.14e100;

lazy_static! {
    static ref PREV_STATUS: Map<char, &'static str> = {
        let mut m: Map<char, &'static str> = Map::new();
        m.insert('B', "ES");
        m.insert('M', "MB");
        m.insert('S', "SE");
        m.insert('E', "BM");
        m
    };
    // static ref FORCE_SPLIT_WORDS:

    static ref RE_HAN: Regex = Regex::new(r"([\x{4E00}-\x{9FD5}]+)").unwrap();
    static ref RE_SKIP: Regex = Regex::new(r"([a-zA-Z0-9]+(?:\.\d+)?%?)").unwrap();

    static ref FORCE_SPLIT_WORDS: Mutex<Set<String>> = Mutex::new(Set::new());
}


fn viterbi(
    obs: &str,
    states: &str,
    start_p: &ProbStart,
    trans_p: &ProbTrans,
    emit_p: &ProbEmit,
) -> (f64, Vec<char>) {
    let mut v: Vec<ProbStart> = vec![Map::new()];
    let mut path: Map<char, Vec<char>> = Map::new();
    for y in states.chars() {
        if let Some(ob) = emit_p[&y].get(&obs.chars().nth(0).unwrap()) {
            v[0].insert(y, start_p[&y] + ob);
        } else {
            v[0].insert(y, start_p[&y] + MIN_FLOAT);
        };

        path.insert(y, vec![y]);
    }

    for t in 1..obs.chars().count() {
        v.push(Map::new());
        let mut newpath: Map<char, Vec<char>> = Map::new();
        for y in states.chars() {
            let em_p = if let Some(ob) = emit_p[&y].get(&obs.chars().nth(t).unwrap()) {
                *ob
            } else {
                MIN_FLOAT
            };

            let xs: Vec<(f64, char)> = PREV_STATUS[&y]
                .chars()
                .map(|y0| {
                    if let Some(ob) = trans_p[&y0].get(&y) {
                        return (v[t - 1][&y0] + ob + em_p, y0);
                    } else {
                        return (v[t - 1][&y0] + MIN_FLOAT + em_p, y0);
                    };
                })
                .collect();

            let (prob, state) = *xs.iter()
                .max_by(|x, y| x.0.partial_cmp(&y.0).unwrap())
                .unwrap();

            // let mut m: ProbStart = Map::new();
            // m.insert(y, prob);
            // v.push(m);

            v[t].insert(y, prob);
            let mut tpath = path[&state].clone();
            tpath.push(y);
            newpath.insert(y, tpath);
        }
        path = newpath;
    }
    let (prob, state) = "ES".chars()
        .map(|y| (v[obs.chars().count() - 1][&y], y))
        .max_by(|x, y| x.0.partial_cmp(&y.0).unwrap())
        .unwrap();

    (prob, path[&state].clone())
}

fn __cut(sentence: &str) -> Vec<String> {
    let (_prob, pos_list) = viterbi(
        sentence,
        "BMES",
        &*prob_start::P,
        &*prob_trans::P,
        &*prob_emit::P,
    );

    // println!("{}-{:?}", prob, pos_list);
    // println!("{:?}", &prob_start::P);
    // vec!["fuck"]
    let (mut begin, mut nexti) = (0, 0);
    let mut segs: Vec<String> = Vec::new();
    for (i, ch) in sentence.chars().enumerate() {
        let pos = pos_list[i];
        if pos == 'B' {
            begin = i;
        } else if pos == 'E' {
            segs.push(char_slice(sentence, begin, i + 1).to_string());
            nexti = i + 1;
        } else if pos == 'S' {
            segs.push(ch.to_string());
            nexti = i + 1;
        }
    }

    if nexti < sentence.chars().count() {
        segs.push(char_slice(sentence, nexti, sentence.chars().count()).to_string());
    }

    segs
}

pub fn cut(sentence: &str) -> Vec<String> {
    // let re_han = Regex::new(r"([\x{4E00}-\x{9FD5}]+)").unwrap();
    // let re_skip = Regex::new(r"([a-zA-Z0-9]+(?:\.\d+)?%?)").unwrap();
    let (re_han, re_skip) = (&*RE_HAN, &*RE_SKIP);
    let blocks = SplitCaptures::new(&re_han, &sentence);
    let mut segs: Vec<String> = Vec::new();
    for blk in blocks {
        match blk {
            SplitState::Captured(caps) => {
                // println!("{:?}", caps);
                for word in __cut(&caps[0]) {
                    // TODO: Force split words
                    segs.push(word.to_string());
                }
            }
            SplitState::Unmatched(t) => {
                let tmp = SplitCaptures::new(&re_skip, &t);
                for x in tmp {
                    match x {
                        SplitState::Captured(caps) => segs.push(caps[0].to_string()),
                        SplitState::Unmatched(t) => segs.push(t.to_string()),
                    }
                }
            }
        }
    }
    // println!("segs in cut = {:?}", segs);
    segs
}

pub fn add_force_split(word: &str) {
    FORCE_SPLIT_WORDS.lock().insert(word.to_string());
}
