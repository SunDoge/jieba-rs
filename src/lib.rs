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
const DEFAULT_DICT: Option<String> = None;

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
    dictionary: Option<String>,
    freq: Map<String, usize>,
}

impl Tokenizer {
    pub fn new(dictionary: Option<&str>) -> Tokenizer {
        match dictionary {
            Some(dict) => {
                Tokenizer {
                    total: 0,
                    initialized: false,
                    dictionary: Some(get_abs_path(dict)),
                    freq: Map::new(),
                }
            }
            None => {
                Tokenizer {
                    total: 0,
                    initialized: false,
                    dictionary: None,
                    freq: Map::new(),
                }
            }
        }
    }

    pub fn gen_pfdict(&self, f: &File) {}

    pub fn get_dict_file(&mut self) -> File {
        match self.dictionary {
            Some(ref dict) => File::open(dict).unwrap(),
            None => File::open(get_abs_path(DEFAULT_DICT_NAME)).unwrap(),
        }
    }

    pub fn initialize(&mut self, dictionary: Option<&str>) {
        let abs_path = if let Some(dict) = dictionary {
            let _abs_path = get_abs_path(&dict);
            if self.dictionary == Some(_abs_path.clone()) && self.initialized {
                return;
            } else {
                self.dictionary = Some(_abs_path.clone());
                self.initialized = true;
            }
            _abs_path
        } else {
            self.dictionary.clone().unwrap()
        };

        println!("{:?}", &abs_path);
    }

    pub fn check_initialized(&mut self) {
        if !self.initialized {
            self.initialize(None);
        }
    }

    pub fn calc(&self,
                sentence: &str,
                DAG: Map<usize, Vec<usize>>,
                route: &mut Map<usize, (usize, usize)>) {
        let n = sentence.chars().count();
        route.insert(n, (0, 0));

    }

    pub fn get_dag(&mut self, sentence: &str) {
        self.check_initialized();
        // let mut DAG = Map::new();
        let n = sentence.chars().count();
        for k in 0..n {
            // let mut tmplist = Vec::new();
            let mut i = k;

        }
    }

    fn cut_all(&self, sentence: &str) {}

    fn cut_dag(&self, sentence: &str) {}

    fn cut_dag_no_hmm(&self, sentence: &str) {}

    /// The main function that segments an entire sentence that contains
    /// Chinese characters into seperated words.
    ///
    /// Parameter:
    ///     - sentence: The str(unicode) to be segmented.
    ///     - cut_all: Model type. True for full pattern, False for accurate pattern.
    ///     - HMM: Whether to use the Hidden Markov Model.
    pub fn cut(&self, sentence: &str, cut_all: bool, hmm: bool) {
        // sentence = strdecode(&sentence);

        let (re_han, re_skip) = if cut_all {
            (Regex::new(r"([\x{4E00}-\x{9FD5}]+)").unwrap(),
             Regex::new(r"[^a-zA-Z0-9+#\n]").unwrap())
        } else {
            (Regex::new(r"([\x{4E00}-\x{9FD5}a-zA-Z0-9+#&\._%]+)").unwrap(),
             Regex::new(r"(\r\n|\s)").unwrap())
        };

        let cut_block = if cut_all {

        } else if hmm {

        } else {

        };

        let cap = re_han.captures(&sentence);
        println!("{:?}", cap);

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
