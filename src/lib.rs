pub struct LocWord {
    word: String,
    begin: usize,
    end: usize,
}

pub struct Jieba {}

impl Jieba {
    pub fn cut(&self, sentence: &str, words: &mut Vec<String>, hmm: bool) {}

    pub fn cut_all(&self, sentence: &str, words: &mut Vec<String>) {}

    pub fn cut_for_search(&self, sentence: &str, words: &mut Vec<String>, hmm: bool) {}

    pub fn cut_hmm(&self, sentence: &str, words: &mut Vec<String>) {}

    pub fn cut_small(&self, sentence: &str, words: &mut Vec<String>, max_word_len: usize) {}

    pub fn tag(&self, sentence: &str, words: &mut Vec<(String, String)>) {}
}
