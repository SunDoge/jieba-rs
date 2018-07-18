#![feature(use_extern_macros, proc_macro, wasm_custom_section, wasm_import_module)]
extern crate wasm_bindgen;

pub mod dict_trie;
pub mod trie;

use wasm_bindgen::prelude::*;

pub struct LocWord {
    word: String,
    begin: usize,
    end: usize,
}

#[wasm_bindgen]
pub struct Jieba {}

#[wasm_bindgen]
impl Jieba {
    pub fn cut(&self, sentence: &str, words: &mut Vec<String>, hmm: bool) {}

    pub fn cut_all(&self, sentence: &str, words: &mut Vec<String>) {}

    pub fn cut_for_search(&self, sentence: &str, words: &mut Vec<String>, hmm: bool) {}

    pub fn cut_hmm(&self, sentence: &str, words: &mut Vec<String>) {}

    pub fn cut_small(&self, sentence: &str, words: &mut Vec<String>, max_word_len: usize) {}

    pub fn tag(&self, sentence: &str, words: &mut Vec<(String, String)>) {}
}



