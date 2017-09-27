#![feature(generators, generator_trait)]
extern crate regex;
extern crate num_cpus;

pub mod analyse;
pub mod posseg;
mod compact;

use std::fs::File;
use regex::Regex;
use std::collections::HashMap as Map;
use std::env;
// use std::io;
use std::error::Error;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

use std::ops::{Generator, GeneratorState};
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
    total: u32,
    initialized: bool,
    dictionary: Option<String>,
    freq: Map<String, u32>,
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

    pub fn gen_pfdict(&self, f: &str) -> (Map<String, u32>, u32) {
        let mut lfreq = Map::new();
        let mut ltotal = 0;
        // let f_name =
        // let mut contents = String::new();
        // f.read_to_string(&mut contents);
        for (lineno, line) in f.lines().enumerate() {
            // TODO: error handle
            let line = line.trim();
            // println!("line: {}", line);
            let v: Vec<&str> = line.split(' ').collect();
            let word = v[0];
            let freq: u32 = v[1].parse().unwrap();
            // println!("{} : {}", word, freq);
            lfreq.insert(word.to_string(), freq);
            ltotal += freq;

            for ch in word.char_indices() {
                // let mut word = word;
                let wfrag = word[..ch.0 + ch.1.len_utf8()].to_string();
                // println!("wfrag = {}", &wfrag);
                if !lfreq.contains_key(&wfrag) {
                    lfreq.insert(wfrag, 0);
                }

            }
        }
        (lfreq, ltotal)
    }

    pub fn get_dict_file(&mut self) -> Result<String, Box<Error>> {
        // match self.dictionary {
        //     Some(ref dict) => File::open(dict)?,
        //     None => File::open(get_abs_path(DEFAULT_DICT_NAME))?,
        // }
        let mut f = if let Some(ref dict) = self.dictionary {
            File::open(dict)?
        } else {
            File::open(get_abs_path(DEFAULT_DICT_NAME))?
        };
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
        Ok(contents)
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
            Some(_abs_path)
        } else {
            self.dictionary.clone()
        };

        println!("abs_path = {:?}", &abs_path);
        let contents = self.get_dict_file().unwrap();
        let (freq, total) = self.gen_pfdict(&contents);
        self.freq = freq;
        self.total = total;
        self.initialized = true;
    }

    pub fn check_initialized(&mut self) {
        if !self.initialized {
            self.initialize(None);
        }
    }

    pub fn calc(&self,
                sentence: &str,
                dag: &Map<usize, Vec<usize>>,
                route: &mut Map<usize, (f64, usize)>) {
        let n = sentence.chars().count();
        route.insert(n, (0.0, 0));
        let logtotal = (self.total as f64).ln();
        for idx in (0..n).rev() {
            // route.insert(idx)
            let xs: Vec<(f64, usize)> = dag[&idx]
                .iter()
                .map(|&x| {
                    let logfreq = if let Some(&freq) = self.freq
                        .get(&sentence[sentence.char_indices().nth(idx).unwrap().0..
                              sentence.char_indices().nth(x).unwrap().0 +
                              sentence.char_indices().nth(x).unwrap().1.len_utf8()]) {
                        (freq as f64).ln()
                    } else {
                        0.0
                    };

                    (logfreq - logtotal + route[&(x + 1)].0, x)
                })
                .collect();

            let max: (f64, usize) =
                *xs.iter().max_by(|x, y| x.0.partial_cmp(&y.0).unwrap()).unwrap();
            // println!("{:?}", &max);
            // println!("{:?}", route[&(idx + 1)]);

            route.insert(idx, max);

        }
        // println!("{:?}", &route);
    }

    pub fn get_dag(&mut self, sentence: &str) -> Map<usize, Vec<usize>> {
        self.check_initialized();
        let mut dag = Map::new();
        let n = sentence.chars().count();
        for k in 0..n {
            let mut tmplist = Vec::new();
            let mut i = k;
            let mut frag = sentence.chars().nth(k).unwrap().to_string();

            // The i must < n - 1 due to the difference between rust and python
            while i < n && self.freq.contains_key(&frag) {

                if self.freq[&frag] > 0 {
                    tmplist.push(i);
                }
                i += 1;
                let _i = if i < n { i } else { n - 1 };
                frag = sentence[sentence.char_indices().nth(k).unwrap().0..
                       sentence.char_indices().nth(_i).unwrap().0 +
                       sentence.char_indices().nth(_i).unwrap().1.len_utf8()]
                    .to_string();

            }
            if tmplist.is_empty() {
                tmplist.push(k);
            }
            dag.insert(k, tmplist);
            // println!("k={}", k);
        }
        dag
    }

    fn cut_all(&mut self, sentence: &str) {
        let dag = self.get_dag(&sentence);
        let old_j = -1;
        for (k, l) in dag {
            println!("{:?}", l);
        }
    }

    fn cut_dag(&mut self, sentence: &str) {
        let dag = self.get_dag(&sentence);
        let mut route: Map<usize, (f64, usize)> = Map::new();
        self.calc(&sentence, &dag, &mut route);
    }

    fn cut_dag_no_hmm(&mut self, sentence: &str) {}

    /// The main function that segments an entire sentence that contains
    /// Chinese characters into seperated words.
    ///
    /// Parameter:
    /// - sentence: The str(unicode) to be segmented.
    /// - cut_all: Model type. True for full pattern, False for accurate pattern.
    /// - HMM: Whether to use the Hidden Markov Model.
    pub fn cut(&mut self, sentence: &str, cut_all: bool, hmm: bool) {
        // sentence = strdecode(&sentence);

        let (re_han, re_skip) = if cut_all {
            (Regex::new(r"([\x{4E00}-\x{9FD5}]+)").unwrap(),
             Regex::new(r"[^a-zA-Z0-9+#\n]").unwrap())
        } else {
            (Regex::new(r"([\x{4E00}-\x{9FD5}a-zA-Z0-9+#&\._%]+)").unwrap(),
             Regex::new(r"(\r\n|\s)").unwrap())
        };

        let cut_block = if cut_all {
            Self::cut_all
        } else if hmm {
            Self::cut_dag
        } else {
            Self::cut_dag_no_hmm
        };
        // let cut_block = Self::cut_all;

        for blk in re_han.captures_iter(&sentence) {
            println!("blk = {:?}", &blk[1]);
            cut_block(self, &blk[1]);
        }

    }
}

pub fn enable_parallel(processnum: usize) {
    let processnum = if processnum == 0 {
        num_cpus::get()
    } else {
        processnum
    };
    println!("cpus: {}", processnum);
}

pub fn disable_parallel() {}



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
