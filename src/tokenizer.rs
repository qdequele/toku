#![allow(soft_unstable)]

use unicode_segmentation::UnicodeSegmentation;
use whatlang::Lang;
use jieba_rs::Jieba;

use crate::stopwords::StopWords;

pub struct TokenizerBuilder;

pub struct Tokenizer<'a> {
    locale: Option<Lang>,
    words: Box<dyn Iterator<Item = &'a str> + 'a>,
}

const TEXT_LANG_TRUNCATE_OVER_CHARS: usize = 200;
const TEXT_LANG_DETECT_PROCEED_OVER_CHARS: usize = 20;
const TEXT_LANG_DETECT_NGRAM_UNDER_CHARS: usize = 60;

impl TokenizerBuilder {
    pub fn from(text: &str) -> Result<Tokenizer, ()> {
        let locale = Self::detect_lang(text);

        Ok(Tokenizer::new(text, locale))
    }

    fn detect_lang(text: &str) -> Option<Lang> {
        if text.len() < TEXT_LANG_DETECT_PROCEED_OVER_CHARS {
            return None;
        }

        let safe_text = if text.len() > TEXT_LANG_TRUNCATE_OVER_CHARS {
            text.char_indices()
                .nth(TEXT_LANG_TRUNCATE_OVER_CHARS)
                .map(|(end_index, _)| &text[0..end_index])
                .unwrap_or(text)
        } else {
            text
        };

        if safe_text.len() < TEXT_LANG_DETECT_NGRAM_UNDER_CHARS {
            Self::detect_lang_slow(safe_text)
        } else {
            Self::detect_lang_fast(safe_text)
        }
    }

    fn detect_lang_slow(safe_text: &str) -> Option<Lang> {
        match whatlang::detect(safe_text) {
            Some(detector) => {
                let mut locale = detector.lang();

                if !detector.is_reliable() {
                    if let Some(alternate_locale) =
                        StopWords::detect_lang(safe_text, detector.script())
                    {
                        locale = alternate_locale;
                    }
                }

                Some(locale)
            }
            None => None,
        }
    }

    fn detect_lang_fast(safe_text: &str) -> Option<Lang> {
        match whatlang::detect_script(safe_text) {
            Some(script) => {
                if let Some(locale) = StopWords::detect_lang(safe_text, script) {
                    Some(locale)
                } else {
                    whatlang::detect_lang(safe_text)
                }
            }
            None => None,
        }
    }
}

impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a str, locale: Option<Lang>) -> Tokenizer<'a> {
        let words = if let Some(Lang::Cmn) = locale {
            let jieba = Jieba::new();
            jieba.cut(text, false).into_iter()
        } else {
            text.split_word_bounds().collect::<Vec<&'a str>>().into_iter()
        };

        Tokenizer {
            locale,
            words: Box::new(words),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(word) = self.words.next() {
            let word = word.to_lowercase();

            if word.contains(" ") {
                continue;
            }
            if !StopWords::match_lang(&word, self.locale) {
                return Some(word);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_cleans_token_english() {
        let mut token_cleaner = TokenizerBuilder::from(
            "The quick brown fox jumps over the lazy dog!",
        )
        .unwrap();

        assert_eq!(token_cleaner.locale, Some(Lang::Eng));
        assert_eq!(
            token_cleaner.next(),
            Some("quick".to_string())
        );
        assert_eq!(
            token_cleaner.next(),
            Some("brown".to_string())
        );
        assert_eq!(token_cleaner.next(), Some("fox".to_string()));
        assert_eq!(token_cleaner.next(), Some("jumps".to_string()));
        assert_eq!(token_cleaner.next(), Some("lazy".to_string()));
        assert_eq!(token_cleaner.next(), Some("dog".to_string()));
        assert_eq!(token_cleaner.next(), Some("!".to_string()));
        assert_eq!(token_cleaner.next(), None);
    }

    #[test]
    fn it_cleans_token_french() {
        let mut token_cleaner = TokenizerBuilder::from(
            "Le vif renard brun saute par dessus le chien paresseux.",
        )
        .unwrap();

        assert_eq!(token_cleaner.locale, Some(Lang::Fra));
        assert_eq!(
            token_cleaner.next(),
            Some("renard".to_string())
        );
        assert_eq!(token_cleaner.next(), Some("brun".to_string()));
        assert_eq!(
            token_cleaner.next(),
            Some("saute".to_string())
        );
        assert_eq!(
            token_cleaner.next(),
            Some("chien".to_string())
        );
        assert_eq!(
            token_cleaner.next(),
            Some("paresseux".to_string())
        );
        assert_eq!(
            token_cleaner.next(),
            Some(".".to_string())
        );
        assert_eq!(token_cleaner.next(), None);
    }

    #[test]
    fn it_cleans_token_chinese() {
        let mut token_cleaner = TokenizerBuilder::from(
            "快狐跨懒狗快狐跨懒狗",
        )
        .unwrap();

        assert_eq!(token_cleaner.locale, Some(Lang::Cmn));
        assert_eq!(token_cleaner.next(), Some("快".to_string()));
        assert_eq!(token_cleaner.next(), Some("狐".to_string()));
        assert_eq!(token_cleaner.next(), Some("跨".to_string()));
        assert_eq!(token_cleaner.next(), Some("懒".to_string()));
        assert_eq!(token_cleaner.next(), Some("狗".to_string()));
        assert_eq!(token_cleaner.next(), Some("快".to_string()));
        assert_eq!(token_cleaner.next(), Some("狐".to_string()));
        assert_eq!(token_cleaner.next(), Some("跨".to_string()));
        assert_eq!(token_cleaner.next(), Some("懒".to_string()));
        assert_eq!(token_cleaner.next(), Some("狗".to_string()));
        assert_eq!(token_cleaner.next(), None);
    }

    #[test]
    fn it_cleans_token_emojis() {
        let mut token_cleaner =
            TokenizerBuilder::from("🚀 🙋‍♂️🙋‍♂️🙋‍♂️")
                .unwrap();

        assert_eq!(token_cleaner.locale, None);
        assert_eq!(token_cleaner.next(), Some("🚀".to_string()));
        assert_eq!(token_cleaner.next(), Some("🙋‍♂️".to_string()));
        assert_eq!(token_cleaner.next(), Some("🙋‍♂️".to_string()));
        assert_eq!(token_cleaner.next(), Some("🙋‍♂️".to_string()));
    }
}