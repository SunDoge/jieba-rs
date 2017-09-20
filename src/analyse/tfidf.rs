use std::collections::HashSet as Set;
use std::collections::HashMap as Map;
use super::super::Tokenizer;

pub const STOP_WORDS: [&str; 32] = ["the", "of", "is", "and", "to", "in", "that", "we", "for",
                                    "an", "are", "by", "be", "as", "on", "with", "can", "if",
                                    "from", "which", "you", "it", "this", "then", "at", "have",
                                    "all", "not", "one", "has", "or", "that"];


trait KeywordExtractor {
    fn set_stop_words(&mut self, stop_words_path: &str) {}
}

struct IDFLoader {
    path: String,
    idf_freq: Map<String, f64>,
    median_idf: f64,
}

impl IDFLoader {
    // fn new(idf_path: &str) -> IDFLoader {

    // }
}


struct TFIDF {
    stop_words: Set<String>,
    tokenizer: Tokenizer,
}
