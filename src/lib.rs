pub mod analyse;
pub mod posseg;
mod compact;

use std::fs::File;


const DEFAULT_DICT_NAME: &'static str = "dict.txt";

pub struct Tokenizer {
    // dictionary:
    total: i32,
}

impl Tokenizer {
    pub fn new(dictionary: &str) -> Tokenizer {
        Tokenizer { total: 0 }
    }

    /// The main function that segments an entire sentence that contains
    /// Chinese characters into seperated words.
    ///
    /// Parameter:
    ///     - sentence: The str(unicode) to be segmented.
    ///     - cut_all: Model type. True for full pattern, False for accurate pattern.
    ///     - HMM: Whether to use the Hidden Markov Model.
    pub fn cut(&self, sentense: &str, cut_all: bool, HMM: bool) {
        
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
