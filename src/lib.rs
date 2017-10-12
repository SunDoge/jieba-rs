extern crate md5;
extern crate num_cpus;
extern crate regex;
extern crate serde_json;
extern crate parking_lot;

#[macro_use]
extern crate lazy_static;

pub mod analyse;
pub mod posseg;
mod compact;
pub mod finalseg;

// use std::fs;
use std::fs::{metadata, File};
use regex::Regex;
use std::collections::BTreeMap as Map;
use std::env;
// use std::io;
use std::error::Error;
use std::io::prelude::*;
use std::sync::{Arc};
use parking_lot::{ReentrantMutex, Mutex, RwLock};
use std::time;
// use std::path::Path;

// use std::path;

use compact::{char_slice, SplitCaptures, SplitState};


const DEFAULT_DICT_NAME: &'static str = "dict.txt";
const DEFAULT_DICT: Option<&str> = None;
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");


lazy_static! {
    static ref DT: Mutex<Tokenizer> = Mutex::new(Tokenizer::new(DEFAULT_DICT));

    static ref RE_HAN_CUT_ALL: Regex = Regex::new(r"([\x{4E00}-\x{9FD5}]+)").unwrap();

    static ref RE_SKIP_CUT_ALL: Regex = Regex::new(r"[^a-zA-Z0-9+#\n]").unwrap();

    static ref RE_HAN_DEFAULT: Regex = Regex::new(r"([\x{4E00}-\x{9FD5}a-zA-Z0-9+#&\._%]+)").unwrap();

    static ref RE_SKIP_DEFAULT: Regex = Regex::new(r"(\r\n|\s)").unwrap();

    static ref RE_ENG: Regex = Regex::new(r"[a-zA-Z0-9]").unwrap();

    static ref RE_USERDICT: Regex = Regex::new(r"^(.+?)( [0-9]+)?( [a-z]+)?$").unwrap();

    static ref DICT_WRITING: Arc<RwLock<Map<String, ReentrantMutex<()>>>> = Arc::new(RwLock::new(Map::new()));
    
}


pub fn get_abs_path(path: &str) -> String {
    let mut cwd = env::current_dir().unwrap();
    cwd.push("src");
    cwd.push(path);
    cwd.to_str().unwrap().to_string()
}

pub struct Tokenizer {
    lock: ReentrantMutex<()>,
    total: u32,
    initialized: bool,
    dictionary: Option<String>,
    freq: Map<String, u32>,
    cache_file: Option<String>,
}

impl Tokenizer {
    pub fn new(dictionary: Option<&str>) -> Tokenizer {
        match dictionary {
            Some(dict) => Tokenizer {
                total: 0,
                initialized: false,
                dictionary: Some(get_abs_path(dict)),
                freq: Map::new(),
                cache_file: None,
                lock: ReentrantMutex::new(())
            },
            None => Tokenizer {
                total: 0,
                initialized: false,
                dictionary: None,
                freq: Map::new(),
                cache_file: None,
                lock: ReentrantMutex::new(())
            },
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
        let mut abs_path = if let Some(dict) = dictionary {
            let mut _abs_path = get_abs_path(&dict);
            if self.dictionary == Some(_abs_path.clone()) && self.initialized {
                return;
            } else {
                self.dictionary = Some(_abs_path.clone());
                self.initialized = false;
            }
            Some(_abs_path)
        } else {
            None
        };

        // println!("abs_path = {:?}", &abs_path);
        // let abs_path = if dictionary.is_some() {
        //     let _abs_path = get_abs_path(dictionary.unwrap());
        //     if self.dictionary == Some(_abs_path) && self.initialized {
        //         return;
        //     } else {
        //         self.dictionary = Some(_abs_path);
        //         self.initialized = false;
        //     }
        // } else {

        // };


        // with self.lock
        {
            *self.lock.lock();
                
                // with DICT_WIRTING[abs_path]
                // if let Some(lock) = DICT_WRITING.read().get(&abs_path.clone().unwrap_or("".to_string())) {
                //     *lock.lock()
                // }

            if self.initialized {
                return;
            }

            // println!("abs_path = {:?}", &abs_path);

            // default_logger
            let t1 = time::Instant::now();
            let cache_file = if self.cache_file.is_some() {
                self.cache_file.clone().unwrap()
            } else if abs_path.is_none() {
                "jiebars.cache".to_string()
            } else {
                // moved?
                let _abs_path = abs_path.clone().unwrap();
                format!("jiebars.u{:x}.cache", md5::compute(&_abs_path))
            };

            let mut tmpdir = env::temp_dir();
            tmpdir.push(&cache_file);
            // println!("cache: {:?}", &tmpdir);
            let mut load_from_cache_fail = true;

            // println!("is file {}", metadata(&tmpdir)?.is_file());

            // if let Ok(m) = metadata(&tmpdir) {
            //     if m.is_file() {
            //         if abs_path.is_some() {
            //             if let Ok(abs) = metadata(&abs_path.clone().unwrap()) {
            //                 if m.modified().unwrap() < abs.modified().unwrap() {}
            //             }
            //         }

            if (metadata(&tmpdir).is_ok() && metadata(&tmpdir).unwrap().is_file()) && (abs_path.is_none() || metadata(&abs_path.clone().unwrap()).is_ok() && metadata(&tmpdir).unwrap().modified().unwrap() > metadata(&abs_path.clone().unwrap()).unwrap().modified().unwrap()) {

                println!("cache");
                let cf = File::open(&tmpdir);

                match cf {
                    Ok(mut t) => {
                        let mut contents = String::new();
                        t.read_to_string(&mut contents).unwrap();
                        let (freq, total): (Map<String, u32>, u32) =
                            serde_json::from_str(&contents).unwrap();
                        self.freq = freq;
                        self.total = total;
                        load_from_cache_fail = false;
                        println!("read from cache: {:?}", &tmpdir);
                    }
                    Err(e) => {
                        load_from_cache_fail = true;
                        println!("{}", e.to_string());
                    }
                }
                
            }

            // println!("bool {}", load_from_cache_fail);
            if load_from_cache_fail {
                
                    
                


                // println!("abs_path = {:?}", &abs_path);
                let contents = self.get_dict_file().unwrap();
                let (freq, total) = self.gen_pfdict(&contents);
                // println!("{:?}", &freq);
                self.freq = freq;
                self.total = total;

                let fd = File::create(&tmpdir);
                println!("tmpdir: {:?}", &tmpdir);
                match fd {
                    Ok(mut t) => {
                        let data = (self.freq.clone(), self.total.clone());
                        let contents = serde_json::to_string(&data).unwrap();
                        t.write_all(&contents.into_bytes()).unwrap();
                        println!("dump to cache: {:?}", &tmpdir);
                    }
                    Err(e) => {
                        println!("{}", e.to_string());
                    }
                }
            }

            self.initialized = true;
            println!(
                "Loading model cost {}.{} seconds",
                t1.elapsed().as_secs(),
                t1.elapsed().subsec_nanos()
            );
        }

        
    }

    pub fn check_initialized(&mut self) {
        if !self.initialized {
            self.initialize(None);
        }
    }

    pub fn calc(
        &self,
        sentence: &str,
        dag: &Map<usize, Vec<usize>>,
        route: &mut Map<usize, (f64, usize)>,
    ) {
        let n = sentence.chars().count();
        route.insert(n, (0.0, 0));
        let logtotal = (self.total as f64).ln();
        for idx in (0..n).rev() {
            // route.insert(idx)
            let xs: Vec<(f64, usize)> = dag[&idx]
                .iter()
                .map(|&x| {
                    let logfreq = if let Some(&freq) = self.freq.get(char_slice(&sentence, idx, x + 1)) {
                        (freq as f64).ln()
                    } else {
                        0.0
                    };

                    (logfreq - logtotal + route[&(x + 1)].0, x)
                })
                .collect();

            let max: (f64, usize) = *xs.iter()
                .max_by(|x, y| x.0.partial_cmp(&y.0).unwrap())
                .unwrap();
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
                frag = char_slice(sentence, k, _i+1).to_string();
            }
            if tmplist.is_empty() {
                tmplist.push(k);
            }
            dag.insert(k, tmplist);
            // println!("k={}", k);
        }
        dag
    }

    fn cut_all(&mut self, sentence: &str) -> Vec<String> {
        let dag = self.get_dag(&sentence);
        let mut old_j = 0;
        let mut segs: Vec<String> = Vec::new();
        for (k, l) in dag {
            if l.len() == 1 && (old_j == 0 || k > old_j) {
                segs.push(char_slice(sentence, k, l[0] + 1).to_string());
                old_j = l[0];
            } else {
                for j in l {
                    if j > k {
                        segs.push(char_slice(sentence, k, j + 1).to_string());
                        old_j = j;
                    }
                }
            }
        }
        segs
    }

    fn cut_dag(&mut self, sentence: &str) -> Vec<String> {
        let dag = self.get_dag(&sentence);
        let mut route: Map<usize, (f64, usize)> = Map::new();
        self.calc(&sentence, &dag, &mut route);
        // println!("{:?}", &route);
        let mut x = 0;
        let mut buf = String::new();
        let n = sentence.chars().count();
        let mut segs: Vec<String> = Vec::new();
        // println!("while", );
        while x < n {
            let y = route[&x].1 + 1;
            let l_word = char_slice(sentence, x, y);
            if y - x == 1 {
                buf.push_str(l_word);
            // println!("buf = {}", &buf);
            } else {
                if buf.len() > 0 {
                    if buf.len() == 1 {
                        segs.push(buf.clone());
                        buf.clear();
                    } else {
                        // In python, both not None and not 0 are true.
                        if !self.freq.contains_key(&buf) {
                            let recognized = finalseg::cut(&buf);
                            for t in recognized {
                                segs.push(t.to_string());
                            }
                        } else if self.freq[&buf] == 0 {
                            let recognized = finalseg::cut(&buf);
                            for t in recognized {
                                segs.push(t.to_string());
                            }
                        } else {
                            for elem in buf.chars() {
                                segs.push(elem.to_string());
                            }
                        }
                        buf.clear();
                    }
                }
                segs.push(l_word.to_string());
            }
            x = y;
        }
        if buf.len() > 0 {
            if buf.len() == 1 {
                segs.push(buf.clone());
            } else if !self.freq.contains_key(&buf) {
                let recognized = finalseg::cut(&buf);
                for t in recognized {
                    segs.push(t.to_string());
                }
            } else {
                for elem in buf.chars() {
                    segs.push(elem.to_string());
                }
            }
        }

        segs
        // vec!["fuck"]
    }

    fn cut_dag_no_hmm(&mut self, sentence: &str) -> Vec<String> {
        let dag = self.get_dag(&sentence);
        let mut route: Map<usize, (f64, usize)> = Map::new();
        self.calc(&sentence, &dag, &mut route);
        // println!("{:?}", &route);
        let mut x = 0;
        let mut buf = String::new();
        let n = sentence.chars().count();
        let mut segs: Vec<String> = Vec::new();

        while x < n {
            let y = route[&x].1 + 1;
            let l_word = char_slice(sentence, x, y);
            if RE_ENG.is_match(l_word) && l_word.chars().count() == 1 {
                buf.push_str(l_word);
                x = y;
            } else {
                if buf.chars().count() > 0 {
                    segs.push(buf.clone());
                    buf.clear();
                }
                segs.push(l_word.to_string());
                x = y;
            }
        }

        if buf.chars().count() > 0 {
            segs.push(buf.clone());
            buf.clear();
        }
        segs
    }

    /// The main function that segments an entire sentence that contains
    /// Chinese characters into seperated words.
    ///
    /// Parameter:
    /// - sentence: The str(unicode) to be segmented.
    /// - cut_all: Model type. True for full pattern, False for accurate pattern.
    /// - HMM: Whether to use the Hidden Markov Model.
    pub fn cut(&mut self, sentence: &str, cut_all: bool, hmm: bool) -> Vec<String> {
        // sentence = strdecode(&sentence);

        let (re_han, re_skip)= if cut_all {
            (
                // Regex::new(r"([\x{4E00}-\x{9FD5}]+)").unwrap(),
                // Regex::new(r"[^a-zA-Z0-9+#\n]").unwrap(),
                &*RE_HAN_CUT_ALL, &*RE_SKIP_CUT_ALL
            )
        } else {
            (
                // Regex::new(r"([\x{4E00}-\x{9FD5}a-zA-Z0-9+#&\._%]+)").unwrap(),
                // Regex::new(r"(\r\n|\s)").unwrap(),
                &*RE_HAN_DEFAULT, &*RE_SKIP_DEFAULT
            )
        };

        let cut_block = if cut_all {
            Self::cut_all
        } else if hmm {
            Self::cut_dag
        } else {
            Self::cut_dag_no_hmm
        };

        // println!("s = {}", &sentence);
        // for blk in re_han.split(&sentence) {
        //     println!("blk = >{}<", &blk);
        //     // cut_block(self, &blk[1]);
        // }
        // let segs = Vec::new();
        let blocks = SplitCaptures::new(&re_han, &sentence);
        let mut segs: Vec<String> = Vec::new();
        for blk in blocks {
            match blk {
                SplitState::Captured(caps) => {
                    // println!("captured: {:?}", &caps[0]);
                    for word in cut_block(self, &caps[0]) {
                        // println!("{}", &word);
                        segs.push(word.to_string());
                    }
                }

                SplitState::Unmatched(t) => {
                    // println!("unmatched: {:?}", t);
                    let tmp = SplitCaptures::new(&re_skip, &t);
                    for x in tmp {
                        match x {
                            SplitState::Captured(caps) => {
                                // println!("{}", &caps[0]);
                                segs.push(caps[0].to_string());
                            }
                            SplitState::Unmatched(t) => if !cut_all {
                                for xx in t.chars() {
                                    // println!("{}", &xx);
                                    segs.push(xx.to_string());
                                }
                            } else {
                                // println!("{}", &t);
                                segs.push(t.to_string());
                            },
                        }
                    }
                }
            }
        }
        // println!("{:?}", segs);
        segs
    }

    /// Finer segmentation for search engines.
    pub fn cut_for_search(&mut self, sentence: &str, hmm: bool) -> Vec<String> {
        let words = self.cut(sentence, false, hmm);
        let mut segs: Vec<String> = Vec::new();
        for w in words {
            if w.chars().count() > 2 {
                for i in 0..w.chars().count() - 1 {
                    let gram2 = char_slice(&w, i, i + 2);
                    if self.freq.contains_key(gram2) && self.freq[gram2] > 0 {
                        segs.push(gram2.to_string());
                    }
                }
            }

            if w.chars().count() > 3 {
                for i in 0..w.chars().count() - 2 {
                    let gram3 = char_slice(&w, i, i + 3);
                    if self.freq.contains_key(gram3) && self.freq[gram3] > 0 {
                        segs.push(gram3.to_string());
                    }
                }
            }

            segs.push(w.to_string());
        }

        segs
    }


    /// Load personalized dict to improve detect rate.
    ///
    /// Parameter:
    /// - f : A plain text file contains words and their ocurrences.
    ///       Can be a file-like object, or the path of the dictionary file,
    ///       whose encoding must be utf-8.
    ///
    /// Structure of dict file:
    /// word1 freq1 word_type1
    /// word2 freq2 word_type2
    /// ...
    /// Word type may be ignored
    pub fn load_user_dict(&mut self, f_name: &str) -> Result<(), std::io::Error> {
        self.check_initialized();
        let mut contents = String::new();
        File::open(&f_name)?.read_to_string(&mut contents)?;

        for (lineno, ln) in contents.lines().enumerate() {
            let line = ln.trim();

            if line.chars().count() == 0 {
                continue;
            }

            let res = RE_USERDICT.captures(&line);
            println!("{:?}", res);
        }


        Ok(())
    }

    pub fn add_word(&mut self, word: &str, freq: u32, tag: &str) {
        self.check_initialized();
    }

    pub fn del_word(&mut self, word: &str) {
        self.add_word(&word, 0, "");
    }
}



pub fn get_freq(k: &str, d: Option<u32>) -> Option<u32> {
    if let Some(freq) = DT.lock().freq.get(k) {
        Some(*freq)
    } else {
        d
    }
}

pub fn cut(sentence: &str, cut_all: bool, hmm: bool) -> Vec<String> {
    DT.lock().cut(sentence, cut_all, hmm)
}

pub fn cut_for_search(sentence: &str, hmm: bool) -> Vec<String> {
    DT.lock().cut_for_search(sentence, hmm)
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
