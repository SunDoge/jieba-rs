extern crate regex;

pub mod analyse;
pub mod posseg;
mod compact;

use std::fs::File;
use regex::Regex;
use std::collections::HashMap as Map;
use std::env;
// use std::path;


const DEFAULT_DICT_NAME: &'static str = "dict.txt";

pub fn get_abs_path(path: &str) -> String {
    let mut cwd = env::current_dir().unwrap();
    cwd.push("src");
    cwd.push(path);
    cwd.to_str().unwrap().to_string()
}

pub struct Tokenizer {
    // dictionary:
    total: i32,
    initialized: bool,
    dictionary: String,
    // freq: Map<String, i32>,
}

impl Tokenizer {
    pub fn new(dictionary: Option<&str>) -> Tokenizer {
        match dictionary {
            Some(dict) => Tokenizer {
                total:0,
                initialized: false,
                dictionary: get_abs_path(dict),

            },
            None => Tokenizer {
                total: 0,
                initialized: false,
                dictionary: String::new(),
            },
        }
    }

    pub fn initialize(&mut self, dictionary: Option<&str>) {
        let mut abs_path = String::new();
        match dictionary {
            Some(path) => {
                abs_path = get_abs_path(path);
                if self.dictionary == abs_path && self.initialized {
                    return;
                } else {
                    self.dictionary = abs_path;
                    self.initialized = true;
                }
            }
            None => abs_path = self.dictionary.clone(),
        }
    }

    pub fn check_initialized(&mut self) {
        if !self.initialized {
            self.initialize(None);
        }
    }

    pub fn get_DAG(&mut self, sentence: &str) {
        self.check_initialized();
        // let mut DAG = Map::new();
        let N = sentence.chars().count();
        for k in 0..N {
            // let mut tmplist = Vec::new();
            let mut i = k;
            let flag = sentence.chars().nth(k).unwrap();
            // while i < N and
        }
    }

    fn cut_all(&self, sentence: &str) {}

    /// The main function that segments an entire sentence that contains
    /// Chinese characters into seperated words.
    ///
    /// Parameter:
    ///     - sentence: The str(unicode) to be segmented.
    ///     - cut_all: Model type. True for full pattern, False for accurate pattern.
    ///     - HMM: Whether to use the Hidden Markov Model.
    pub fn cut(&self, sentence: &mut str, cut_all: bool, HMM: bool) {
        // sentence = strdecode(&sentence);
        let (re_han, re_skip) = if cut_all {
            (Regex::new(r"([\u4E00-\u9FD5]+)").unwrap(), Regex::new(r"[^a-zA-Z0-9+#\n]").unwrap())
        } else {
            (Regex::new(r"([\u4E00-\u9FD5a-zA-Z0-9+#&\._%]+)").unwrap(),
             Regex::new(r"(\r\n|\s)").unwrap())
        };

    }
}





#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_const_vec() {
        use analyse::tfidf;
        assert_eq!(tfidf::STOP_WORDS[0], "the");
    }

    #[test]
    fn test_char_state() {
        // use posseg::char_state_tap::
    }
}
