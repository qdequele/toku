extern crate test;

use jieba_rs::Jieba;
use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::str::FromStr;
// use tokenizers::models::bpe::BPE;
// use tokenizers::tokenizer::{EncodeInput, Result, Tokenizer};
use deunicode::deunicode;
use unicode_segmentation::UnicodeSegmentation;
use whatlang::{Lang, Script};

use crate::stopwords::StopWords;

static JIEBA: Lazy<Jieba> = Lazy::new(Jieba::new);
// static BPE: Lazy<Tokenizer> = Lazy::new(|| {
//     let bpe = BPE::new().build();
//     Tokenizer::new(Box::new(bpe))
// });

#[derive(Debug)]
pub enum LangDetection {
    Forced(Lang),
    AutoWithDefault(Lang),
    Auto,
    None,
}

impl FromStr for LangDetection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Self::Auto),
            "none" => Ok(Self::None),
            "epo" => Ok(Self::Forced(Lang::Epo)),
            "eng" => Ok(Self::Forced(Lang::Eng)),
            "rus" => Ok(Self::Forced(Lang::Rus)),
            "cmn" => Ok(Self::Forced(Lang::Cmn)),
            "spa" => Ok(Self::Forced(Lang::Spa)),
            "por" => Ok(Self::Forced(Lang::Por)),
            "ita" => Ok(Self::Forced(Lang::Ita)),
            "ben" => Ok(Self::Forced(Lang::Ben)),
            "fra" => Ok(Self::Forced(Lang::Fra)),
            "deu" => Ok(Self::Forced(Lang::Deu)),
            "ukr" => Ok(Self::Forced(Lang::Ukr)),
            "kat" => Ok(Self::Forced(Lang::Kat)),
            "arb" => Ok(Self::Forced(Lang::Arb)),
            "hin" => Ok(Self::Forced(Lang::Hin)),
            "jpn" => Ok(Self::Forced(Lang::Jpn)),
            "heb" => Ok(Self::Forced(Lang::Heb)),
            "ydd" => Ok(Self::Forced(Lang::Ydd)),
            "pol" => Ok(Self::Forced(Lang::Pol)),
            "amh" => Ok(Self::Forced(Lang::Amh)),
            "tir" => Ok(Self::Forced(Lang::Tir)),
            "jav" => Ok(Self::Forced(Lang::Jav)),
            "kor" => Ok(Self::Forced(Lang::Kor)),
            "nob" => Ok(Self::Forced(Lang::Nob)),
            "nno" => Ok(Self::Forced(Lang::Nno)),
            "dan" => Ok(Self::Forced(Lang::Dan)),
            "swe" => Ok(Self::Forced(Lang::Swe)),
            "fin" => Ok(Self::Forced(Lang::Fin)),
            "tur" => Ok(Self::Forced(Lang::Tur)),
            "nld" => Ok(Self::Forced(Lang::Nld)),
            "hun" => Ok(Self::Forced(Lang::Hun)),
            "ces" => Ok(Self::Forced(Lang::Ces)),
            "ell" => Ok(Self::Forced(Lang::Ell)),
            "bul" => Ok(Self::Forced(Lang::Bul)),
            "bel" => Ok(Self::Forced(Lang::Bel)),
            "mar" => Ok(Self::Forced(Lang::Mar)),
            "kan" => Ok(Self::Forced(Lang::Kan)),
            "ron" => Ok(Self::Forced(Lang::Ron)),
            "slv" => Ok(Self::Forced(Lang::Slv)),
            "hrv" => Ok(Self::Forced(Lang::Hrv)),
            "srp" => Ok(Self::Forced(Lang::Srp)),
            "mkd" => Ok(Self::Forced(Lang::Mkd)),
            "lit" => Ok(Self::Forced(Lang::Lit)),
            "lav" => Ok(Self::Forced(Lang::Lav)),
            "est" => Ok(Self::Forced(Lang::Est)),
            "tam" => Ok(Self::Forced(Lang::Tam)),
            "vie" => Ok(Self::Forced(Lang::Vie)),
            "urd" => Ok(Self::Forced(Lang::Urd)),
            "tha" => Ok(Self::Forced(Lang::Tha)),
            "guj" => Ok(Self::Forced(Lang::Guj)),
            "uzb" => Ok(Self::Forced(Lang::Uzb)),
            "pan" => Ok(Self::Forced(Lang::Pan)),
            "azj" => Ok(Self::Forced(Lang::Azj)),
            "ind" => Ok(Self::Forced(Lang::Ind)),
            "tel" => Ok(Self::Forced(Lang::Tel)),
            "pes" => Ok(Self::Forced(Lang::Pes)),
            "mal" => Ok(Self::Forced(Lang::Mal)),
            "hau" => Ok(Self::Forced(Lang::Hau)),
            "ori" => Ok(Self::Forced(Lang::Ori)),
            "mya" => Ok(Self::Forced(Lang::Mya)),
            "bho" => Ok(Self::Forced(Lang::Bho)),
            "tgl" => Ok(Self::Forced(Lang::Tgl)),
            "yor" => Ok(Self::Forced(Lang::Yor)),
            "mai" => Ok(Self::Forced(Lang::Mai)),
            "orm" => Ok(Self::Forced(Lang::Orm)),
            "ibo" => Ok(Self::Forced(Lang::Ibo)),
            "ceb" => Ok(Self::Forced(Lang::Ceb)),
            "kur" => Ok(Self::Forced(Lang::Kur)),
            "mlg" => Ok(Self::Forced(Lang::Mlg)),
            "skr" => Ok(Self::Forced(Lang::Skr)),
            "nep" => Ok(Self::Forced(Lang::Nep)),
            "sin" => Ok(Self::Forced(Lang::Sin)),
            "khm" => Ok(Self::Forced(Lang::Khm)),
            "tuk" => Ok(Self::Forced(Lang::Tuk)),
            "som" => Ok(Self::Forced(Lang::Som)),
            "nya" => Ok(Self::Forced(Lang::Nya)),
            "aka" => Ok(Self::Forced(Lang::Aka)),
            "zul" => Ok(Self::Forced(Lang::Zul)),
            "kin" => Ok(Self::Forced(Lang::Kin)),
            "hat" => Ok(Self::Forced(Lang::Hat)),
            "ilo" => Ok(Self::Forced(Lang::Ilo)),
            "run" => Ok(Self::Forced(Lang::Run)),
            "sna" => Ok(Self::Forced(Lang::Sna)),
            "uig" => Ok(Self::Forced(Lang::Uig)),
            "afr" => Ok(Self::Forced(Lang::Afr)),
            "lat" => Ok(Self::Forced(Lang::Lat)),
            "slk" => Ok(Self::Forced(Lang::Slk)),
            _ => Err("".into()),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Precision {
    Hight,
    Low,
}

impl FromStr for Precision {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hight" | "Hight" | "H" => Ok(Self::Hight),
            "low" | "Low" | "L" => Ok(Self::Low),
            _ => Err("".into()),
        }
    }
}

#[derive(Debug)]
pub struct TokenizerBuilder {
    lang_detection: LangDetection,
    keep_ponctuation: bool,
    default_stopwords: bool,
    lowercased: bool,
    unicode: bool,
    added_stop_words: HashSet<String>,
    precision: Precision,
}

impl Default for TokenizerBuilder {
    fn default() -> Self {
        Self {
            lang_detection: LangDetection::Auto,
            keep_ponctuation: true,
            default_stopwords: true,
            lowercased: true,
            unicode: false,
            added_stop_words: HashSet::new(),
            precision: Precision::Low,
        }
    }
}

impl TokenizerBuilder {
    pub fn new() -> TokenizerBuilder {
        TokenizerBuilder::default()
    }

    pub fn lang_detection<'a>(&'a mut self, detection: LangDetection) -> &'a mut Self {
        self.lang_detection = detection;
        self
    }

    pub fn keep_ponctuation<'a>(&'a mut self, keep_ponctuation: bool) -> &'a mut Self {
        self.keep_ponctuation = keep_ponctuation;
        self
    }

    pub fn default_stopwords<'a>(&'a mut self, default_stopwords: bool) -> &'a mut Self {
        self.default_stopwords = default_stopwords;
        self
    }

    pub fn lowercased<'a>(&'a mut self, lowercased: bool) -> &'a mut Self {
        self.lowercased = lowercased;
        self
    }

    pub fn unicode<'a>(&'a mut self, unicode: bool) -> &'a mut Self {
        self.unicode = unicode;
        self
    }

    pub fn added_stop_words<'a>(&'a mut self, added_stop_words: HashSet<String>) -> &'a mut Self {
        self.added_stop_words = added_stop_words;
        self
    }

    pub fn precision<'a>(&'a mut self, precision: Precision) -> &'a mut Self {
        self.precision = precision;
        self
    }

    pub fn build<'a>(self, text: &'a str) -> Tokenizer<'a> {
        let (lang, script) = match self.lang_detection {
            LangDetection::Forced(lang) => (Some(lang), Some(crate::lang_to_script(lang))),
            LangDetection::AutoWithDefault(lang) => {
                let detected_lang = self.detect_lang(&text);
                if let None = detected_lang.0 {
                    (Some(lang), Some(crate::lang_to_script(lang)))
                } else {
                    detected_lang
                }
            }
            LangDetection::Auto => self.detect_lang(&text),
            LangDetection::None => (None, None),
        };

        let words = self.cut_words(text, script);

        let stopwords = if self.default_stopwords == true {
            if let Some(lang) = lang {
                let sw = StopWords::for_lang(lang)
                    .iter()
                    .map(|&x| x.to_string())
                    .collect::<HashSet<String>>();
                sw.intersection(&self.added_stop_words);
                sw
            } else {
                self.added_stop_words
            }
        } else {
            self.added_stop_words
        };

        Tokenizer {
            lowercased: self.lowercased,
            unicode: self.unicode,
            stop_words: stopwords,
            words,
        }
    }

    // Helpers

    fn detect_lang(&self, text: &str) -> (Option<Lang>, Option<Script>) {
        match self.precision {
            Precision::Hight => TokenizerBuilder::detect_lang_slow(text),
            Precision::Low => {
                let test_text = if text.len() > 200 {
                    text.char_indices()
                        .nth(200)
                        .map(|(end_index, _)| &text[0..end_index])
                        .unwrap_or(text)
                } else {
                    text
                };
                TokenizerBuilder::detect_lang_fast(test_text)
            }
        }
    }

    fn cut_words<'a>(
        &self,
        text: &'a str,
        script: Option<Script>,
    ) -> Box<dyn Iterator<Item = &'a str> + 'a> {
        if self.precision == Precision::Hight {
            if let Some(script) = script {
                if script == Script::Mandarin {
                    return Box::new(JIEBA.cut(text, false).into_iter());
                }
            }
        }
        if self.keep_ponctuation {
            return Box::new(
                text.split_word_bounds()
                    .collect::<Vec<&'a str>>()
                    .into_iter(),
            );
        }
        Box::new(text.unicode_words().collect::<Vec<&'a str>>().into_iter())
    }

    fn detect_lang_slow(safe_text: &str) -> (Option<Lang>, Option<Script>) {
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

                (Some(locale), Some(detector.script()))
            }
            None => (None, None),
        }
    }

    fn detect_lang_fast(safe_text: &str) -> (Option<Lang>, Option<Script>) {
        match whatlang::detect_script(safe_text) {
            Some(script) => {
                if let Some(locale) = StopWords::detect_lang(safe_text, script) {
                    (Some(locale), Some(script))
                } else {
                    (whatlang::detect_lang(safe_text), Some(script))
                }
            }
            None => (None, None),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenType {
    Word { stop_word: bool },
    Separator,
}

pub struct Tokenizer<'a> {
    lowercased: bool,
    unicode: bool,
    stop_words: HashSet<String>,
    words: Box<dyn Iterator<Item = &'a str> + 'a>,
}

//TODO: remove word separator "\'\`"
impl<'a> Iterator for Tokenizer<'a> {
    type Item = (TokenType, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(word) = self.words.next() {
            // let word = if self.lowercased {
            //     word.to_lowercase()
            // } else {
            //     word.to_string()
            // };

            if let Some(c) = word.chars().next() {
                if !c.is_alphanumeric() {
                    return Some((TokenType::Separator, word));
                }
            }
            // let word = if self.unicode { deunicode(&word) } else { word };
            let stop_word = self.stop_words.contains(&word.to_lowercase());
            return Some((TokenType::Word { stop_word }, word));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    // #[test]
    // fn it_cleans_token_english() {
    //     let mut tokens =
    //         TokenizerBuilder::new().build("The quick brown fox jumps over the lazy dog!");

    //     assert_eq!(tokens.next(), Some("quick".to_string()));
    //     assert_eq!(tokens.next(), Some("brown".to_string()));
    //     assert_eq!(tokens.next(), Some("fox".to_string()));
    //     assert_eq!(tokens.next(), Some("jumps".to_string()));
    //     assert_eq!(tokens.next(), Some("lazy".to_string()));
    //     assert_eq!(tokens.next(), Some("dog".to_string()));
    //     assert_eq!(tokens.next(), Some("!".to_string()));
    //     assert_eq!(tokens.next(), None);
    // }

    // #[test]
    // fn test_1() {
    //     let mut builder = TokenizerBuilder::new();
    //     builder.lang_detection(LangDetection::Auto);
    //     builder.precision(Precision::Hight);
    //     builder.keep_ponctuation(true);
    //     builder.lowercased(true);
    //     builder.default_stopwords(true);

    //     let mut tokens = builder.build("La position de Lutèce, sur l'île aujourd'hui nommée l'île de la Cité, permettant le franchissement du grand fleuve navigable qu'est la Seine par une voie reliant le Nord et le Sud des Gaules, en fait dès l'Antiquité une cité importante, capitale des Parisii, puis lieu de séjour d'un empereur romain.");

    //     assert_eq!(tokens.next(), Some("position".to_string()));
    //     assert_eq!(tokens.next(), Some("lutèce".to_string()));
    //     assert_eq!(tokens.next(), Some(",".to_string()));
    //     assert_eq!(tokens.next(), Some("nommée".to_string()));
    //     assert_eq!(tokens.next(), Some("hui".to_string()));
    //     assert_eq!(tokens.next(), Some("cité".to_string()));
    //     assert_eq!(tokens.next(), Some("île".to_string()));
    //     assert_eq!(tokens.next(), Some(",".to_string()));
    //     assert_eq!(tokens.next(), Some("permettant".to_string()));
    //     assert_eq!(tokens.next(), Some("franchissement".to_string()));
    //     assert_eq!(tokens.next(), Some("grand".to_string()));
    //     assert_eq!(tokens.next(), Some("fleuve".to_string()));
    //     assert_eq!(tokens.next(), Some("navigable".to_string()));
    //     assert_eq!(tokens.next(), Some("seine".to_string()));
    //     assert_eq!(tokens.next(), Some("est".to_string()));
    //     assert_eq!(tokens.next(), Some("voie".to_string()));
    //     assert_eq!(tokens.next(), Some("reliant".to_string()));
    //     assert_eq!(tokens.next(), Some("nord".to_string()));
    //     assert_eq!(tokens.next(), Some("sud".to_string()));
    //     assert_eq!(tokens.next(), Some("gaules".to_string()));
    //     assert_eq!(tokens.next(), Some(",".to_string()));
    //     assert_eq!(tokens.next(), Some("cité".to_string()));
    //     assert_eq!(tokens.next(), Some("antiquité".to_string()));
    //     assert_eq!(tokens.next(), Some("importante".to_string()));
    //     assert_eq!(tokens.next(), Some(",".to_string()));
    //     assert_eq!(tokens.next(), Some("capitale".to_string()));
    //     assert_eq!(tokens.next(), Some("parisii".to_string()));
    //     assert_eq!(tokens.next(), Some(",".to_string()));
    //     assert_eq!(tokens.next(), Some("lieu".to_string()));
    //     assert_eq!(tokens.next(), Some("séjour".to_string()));
    //     assert_eq!(tokens.next(), Some("empereur".to_string()));
    //     assert_eq!(tokens.next(), Some("un".to_string()));
    //     assert_eq!(tokens.next(), Some("romain".to_string()));
    //     assert_eq!(tokens.next(), Some(".".to_string()));
    //     assert_eq!(tokens.next(), None);
    // }

    #[bench]
    fn bench_fra_lang_auto(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Auto);
            builder.precision(Precision::Low);
            builder.keep_ponctuation(false);
            builder.lowercased(false);
            builder.default_stopwords(false);
            builder.unicode(false);

            let tokens = builder.build("La position de Lutèce, sur l'île aujourd'hui nommée l'île de la Cité, permettant le franchissement du grand fleuve navigable qu'est la Seine par une voie reliant le Nord et le Sud des Gaules, en fait dès l'Antiquité une cité importante, capitale des Parisii, puis lieu de séjour d'un empereur romain.");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_fra_lang_auto_impact_ponctualtion(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Auto);
            builder.precision(Precision::Low);
            builder.keep_ponctuation(true);
            builder.lowercased(false);
            builder.default_stopwords(false);
            builder.unicode(false);

            let tokens = builder.build("La position de Lutèce, sur l'île aujourd'hui nommée l'île de la Cité, permettant le franchissement du grand fleuve navigable qu'est la Seine par une voie reliant le Nord et le Sud des Gaules, en fait dès l'Antiquité une cité importante, capitale des Parisii, puis lieu de séjour d'un empereur romain.");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_fra_lang_auto_impact_lowercased(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Auto);
            builder.precision(Precision::Low);
            builder.keep_ponctuation(false);
            builder.lowercased(true);
            builder.default_stopwords(false);
            builder.unicode(false);

            let tokens = builder.build("La position de Lutèce, sur l'île aujourd'hui nommée l'île de la Cité, permettant le franchissement du grand fleuve navigable qu'est la Seine par une voie reliant le Nord et le Sud des Gaules, en fait dès l'Antiquité une cité importante, capitale des Parisii, puis lieu de séjour d'un empereur romain.");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_fra_lang_auto_impact_unicode(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Auto);
            builder.precision(Precision::Low);
            builder.keep_ponctuation(false);
            builder.lowercased(false);
            builder.default_stopwords(false);
            builder.unicode(true);

            let tokens = builder.build("La position de Lutèce, sur l'île aujourd'hui nommée l'île de la Cité, permettant le franchissement du grand fleuve navigable qu'est la Seine par une voie reliant le Nord et le Sud des Gaules, en fait dès l'Antiquité une cité importante, capitale des Parisii, puis lieu de séjour d'un empereur romain.");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_fra_lang_auto_full_hight(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Auto);
            builder.precision(Precision::Hight);
            builder.keep_ponctuation(true);
            builder.lowercased(true);
            builder.default_stopwords(true);

            let tokens = builder.build("La position de Lutèce, sur l'île aujourd'hui nommée l'île de la Cité, permettant le franchissement du grand fleuve navigable qu'est la Seine par une voie reliant le Nord et le Sud des Gaules, en fait dès l'Antiquité une cité importante, capitale des Parisii, puis lieu de séjour d'un empereur romain.");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_fra_lang_auto_full_low(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Auto);
            builder.precision(Precision::Low);
            builder.keep_ponctuation(true);
            builder.lowercased(true);
            builder.default_stopwords(true);

            let tokens = builder.build("La position de Lutèce, sur l'île aujourd'hui nommée l'île de la Cité, permettant le franchissement du grand fleuve navigable qu'est la Seine par une voie reliant le Nord et le Sud des Gaules, en fait dès l'Antiquité une cité importante, capitale des Parisii, puis lieu de séjour d'un empereur romain.");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_fra_lang_fra(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Forced(Lang::Fra));
            builder.precision(Precision::Low);
            builder.keep_ponctuation(false);
            builder.lowercased(false);
            builder.default_stopwords(false);
            builder.unicode(false);

            let tokens = builder.build("La position de Lutèce, sur l'île aujourd'hui nommée l'île de la Cité, permettant le franchissement du grand fleuve navigable qu'est la Seine par une voie reliant le Nord et le Sud des Gaules, en fait dès l'Antiquité une cité importante, capitale des Parisii, puis lieu de séjour d'un empereur romain.");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_fra_lang_fra_impact_ponctualtion(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Forced(Lang::Fra));
            builder.precision(Precision::Low);
            builder.keep_ponctuation(true);
            builder.lowercased(false);
            builder.default_stopwords(false);
            builder.unicode(false);

            let tokens = builder.build("La position de Lutèce, sur l'île aujourd'hui nommée l'île de la Cité, permettant le franchissement du grand fleuve navigable qu'est la Seine par une voie reliant le Nord et le Sud des Gaules, en fait dès l'Antiquité une cité importante, capitale des Parisii, puis lieu de séjour d'un empereur romain.");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_fra_lang_fra_impact_lowercased(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Forced(Lang::Fra));
            builder.precision(Precision::Low);
            builder.keep_ponctuation(false);
            builder.lowercased(true);
            builder.default_stopwords(false);
            builder.unicode(false);

            let tokens = builder.build("La position de Lutèce, sur l'île aujourd'hui nommée l'île de la Cité, permettant le franchissement du grand fleuve navigable qu'est la Seine par une voie reliant le Nord et le Sud des Gaules, en fait dès l'Antiquité une cité importante, capitale des Parisii, puis lieu de séjour d'un empereur romain.");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_fra_lang_fra_impact_unicode(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Forced(Lang::Fra));
            builder.precision(Precision::Low);
            builder.keep_ponctuation(false);
            builder.lowercased(false);
            builder.default_stopwords(false);
            builder.unicode(true);

            let tokens = builder.build("La position de Lutèce, sur l'île aujourd'hui nommée l'île de la Cité, permettant le franchissement du grand fleuve navigable qu'est la Seine par une voie reliant le Nord et le Sud des Gaules, en fait dès l'Antiquité une cité importante, capitale des Parisii, puis lieu de séjour d'un empereur romain.");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_fra_lang_fra_full_hight(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Forced(Lang::Fra));
            builder.precision(Precision::Hight);
            builder.keep_ponctuation(true);
            builder.lowercased(true);
            builder.default_stopwords(true);

            let tokens = builder.build("La position de Lutèce, sur l'île aujourd'hui nommée l'île de la Cité, permettant le franchissement du grand fleuve navigable qu'est la Seine par une voie reliant le Nord et le Sud des Gaules, en fait dès l'Antiquité une cité importante, capitale des Parisii, puis lieu de séjour d'un empereur romain.");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_fra_lang_fra_full_low(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Forced(Lang::Fra));
            builder.precision(Precision::Low);
            builder.keep_ponctuation(true);
            builder.lowercased(true);
            builder.default_stopwords(true);

            let tokens = builder.build("La position de Lutèce, sur l'île aujourd'hui nommée l'île de la Cité, permettant le franchissement du grand fleuve navigable qu'est la Seine par une voie reliant le Nord et le Sud des Gaules, en fait dès l'Antiquité une cité importante, capitale des Parisii, puis lieu de séjour d'un empereur romain.");
            for _ in tokens{
                continue
            }
        });
    }

    //Anglais

    #[bench]
    fn bench_eng_lang_auto(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Auto);
            builder.precision(Precision::Low);
            builder.keep_ponctuation(false);
            builder.lowercased(false);
            builder.default_stopwords(false);
            builder.unicode(false);

            let tokens = builder.build("The local authority for the City, namely the City of London Corporation, is unique in the UK and has some unusual responsibilities for a local council, such as being the police authority. It is also unusual in having responsibilities and ownerships beyond its boundaries. The Corporation is headed by the Lord Mayor of the City of London (an office separate from, and much older than, the Mayor of London). The Lord Mayor, as of November 2019, is William Russell.[9] ");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_eng_lang_auto_impact_ponctualtion(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Auto);
            builder.precision(Precision::Low);
            builder.keep_ponctuation(true);
            builder.lowercased(false);
            builder.default_stopwords(false);
            builder.unicode(false);

            let tokens = builder.build("The local authority for the City, namely the City of London Corporation, is unique in the UK and has some unusual responsibilities for a local council, such as being the police authority. It is also unusual in having responsibilities and ownerships beyond its boundaries. The Corporation is headed by the Lord Mayor of the City of London (an office separate from, and much older than, the Mayor of London). The Lord Mayor, as of November 2019, is William Russell.[9] ");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_eng_lang_auto_impact_lowercased(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Auto);
            builder.precision(Precision::Low);
            builder.keep_ponctuation(false);
            builder.lowercased(true);
            builder.default_stopwords(false);
            builder.unicode(false);

            let tokens = builder.build("The local authority for the City, namely the City of London Corporation, is unique in the UK and has some unusual responsibilities for a local council, such as being the police authority. It is also unusual in having responsibilities and ownerships beyond its boundaries. The Corporation is headed by the Lord Mayor of the City of London (an office separate from, and much older than, the Mayor of London). The Lord Mayor, as of November 2019, is William Russell.[9] ");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_eng_lang_auto_impact_unicode(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Auto);
            builder.precision(Precision::Low);
            builder.keep_ponctuation(false);
            builder.lowercased(false);
            builder.default_stopwords(false);
            builder.unicode(true);

            let tokens = builder.build("The local authority for the City, namely the City of London Corporation, is unique in the UK and has some unusual responsibilities for a local council, such as being the police authority. It is also unusual in having responsibilities and ownerships beyond its boundaries. The Corporation is headed by the Lord Mayor of the City of London (an office separate from, and much older than, the Mayor of London). The Lord Mayor, as of November 2019, is William Russell.[9] ");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_eng_lang_auto_full_hight(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Auto);
            builder.precision(Precision::Hight);
            builder.keep_ponctuation(true);
            builder.lowercased(true);
            builder.default_stopwords(true);

            let tokens = builder.build("The local authority for the City, namely the City of London Corporation, is unique in the UK and has some unusual responsibilities for a local council, such as being the police authority. It is also unusual in having responsibilities and ownerships beyond its boundaries. The Corporation is headed by the Lord Mayor of the City of London (an office separate from, and much older than, the Mayor of London). The Lord Mayor, as of November 2019, is William Russell.[9] ");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_eng_lang_auto_full_low(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Auto);
            builder.precision(Precision::Low);
            builder.keep_ponctuation(true);
            builder.lowercased(true);
            builder.default_stopwords(true);

            let tokens = builder.build("The local authority for the City, namely the City of London Corporation, is unique in the UK and has some unusual responsibilities for a local council, such as being the police authority. It is also unusual in having responsibilities and ownerships beyond its boundaries. The Corporation is headed by the Lord Mayor of the City of London (an office separate from, and much older than, the Mayor of London). The Lord Mayor, as of November 2019, is William Russell.[9] ");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_eng_lang_eng(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Forced(Lang::Eng));
            builder.precision(Precision::Low);
            builder.keep_ponctuation(false);
            builder.lowercased(false);
            builder.default_stopwords(false);
            builder.unicode(false);

            let tokens = builder.build("The local authority for the City, namely the City of London Corporation, is unique in the UK and has some unusual responsibilities for a local council, such as being the police authority. It is also unusual in having responsibilities and ownerships beyond its boundaries. The Corporation is headed by the Lord Mayor of the City of London (an office separate from, and much older than, the Mayor of London). The Lord Mayor, as of November 2019, is William Russell.[9] ");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_eng_lang_eng_impact_ponctualtion(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Forced(Lang::Eng));
            builder.precision(Precision::Low);
            builder.keep_ponctuation(true);
            builder.lowercased(false);
            builder.default_stopwords(false);
            builder.unicode(false);

            let tokens = builder.build("The local authority for the City, namely the City of London Corporation, is unique in the UK and has some unusual responsibilities for a local council, such as being the police authority. It is also unusual in having responsibilities and ownerships beyond its boundaries. The Corporation is headed by the Lord Mayor of the City of London (an office separate from, and much older than, the Mayor of London). The Lord Mayor, as of November 2019, is William Russell.[9] ");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_eng_lang_eng_impact_lowercased(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Forced(Lang::Eng));
            builder.precision(Precision::Low);
            builder.keep_ponctuation(false);
            builder.lowercased(true);
            builder.default_stopwords(false);
            builder.unicode(false);

            let tokens = builder.build("The local authority for the City, namely the City of London Corporation, is unique in the UK and has some unusual responsibilities for a local council, such as being the police authority. It is also unusual in having responsibilities and ownerships beyond its boundaries. The Corporation is headed by the Lord Mayor of the City of London (an office separate from, and much older than, the Mayor of London). The Lord Mayor, as of November 2019, is William Russell.[9] ");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_eng_lang_eng_impact_unicode(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Forced(Lang::Eng));
            builder.precision(Precision::Low);
            builder.keep_ponctuation(false);
            builder.lowercased(false);
            builder.default_stopwords(false);
            builder.unicode(true);

            let tokens = builder.build("The local authority for the City, namely the City of London Corporation, is unique in the UK and has some unusual responsibilities for a local council, such as being the police authority. It is also unusual in having responsibilities and ownerships beyond its boundaries. The Corporation is headed by the Lord Mayor of the City of London (an office separate from, and much older than, the Mayor of London). The Lord Mayor, as of November 2019, is William Russell.[9] ");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_eng_lang_eng_full_hight(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Forced(Lang::Eng));
            builder.precision(Precision::Hight);
            builder.keep_ponctuation(true);
            builder.lowercased(true);
            builder.default_stopwords(true);

            let tokens = builder.build("The local authority for the City, namely the City of London Corporation, is unique in the UK and has some unusual responsibilities for a local council, such as being the police authority. It is also unusual in having responsibilities and ownerships beyond its boundaries. The Corporation is headed by the Lord Mayor of the City of London (an office separate from, and much older than, the Mayor of London). The Lord Mayor, as of November 2019, is William Russell.[9] ");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_eng_lang_eng_full_low(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Forced(Lang::Eng));
            builder.precision(Precision::Low);
            builder.keep_ponctuation(true);
            builder.lowercased(true);
            builder.default_stopwords(true);

            let tokens = builder.build("The local authority for the City, namely the City of London Corporation, is unique in the UK and has some unusual responsibilities for a local council, such as being the police authority. It is also unusual in having responsibilities and ownerships beyond its boundaries. The Corporation is headed by the Lord Mayor of the City of London (an office separate from, and much older than, the Mayor of London). The Lord Mayor, as of November 2019, is William Russell.[9] ");
            for _ in tokens{
                continue
            }
        });
    }

    // Chinois

    #[bench]
    fn bench_cnn_lang_auto(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Auto);
            builder.precision(Precision::Low);
            builder.keep_ponctuation(false);
            builder.lowercased(false);
            builder.default_stopwords(false);
            builder.unicode(false);

            let tokens = builder.build("距今60万年-2万年的时间内，北京地区处于旧石器时代，在周口店发现了旧石器时代早期北京直立人、中期新洞人和晚期山顶洞人的典型遗址。北京地区在不晚于1万年前已经开始进入新石器时代。当时该地区人类定居生活固定化，逐渐从山洞中迁徙出来，到平原地区定居[12]。 ");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_cnn_lang_auto_impact_ponctualtion(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Auto);
            builder.precision(Precision::Low);
            builder.keep_ponctuation(true);
            builder.lowercased(false);
            builder.default_stopwords(false);
            builder.unicode(false);

            let tokens = builder.build("距今60万年-2万年的时间内，北京地区处于旧石器时代，在周口店发现了旧石器时代早期北京直立人、中期新洞人和晚期山顶洞人的典型遗址。北京地区在不晚于1万年前已经开始进入新石器时代。当时该地区人类定居生活固定化，逐渐从山洞中迁徙出来，到平原地区定居[12]。 ");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_cnn_lang_auto_impact_lowercased(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Auto);
            builder.precision(Precision::Low);
            builder.keep_ponctuation(false);
            builder.lowercased(true);
            builder.default_stopwords(false);
            builder.unicode(false);

            let tokens = builder.build("距今60万年-2万年的时间内，北京地区处于旧石器时代，在周口店发现了旧石器时代早期北京直立人、中期新洞人和晚期山顶洞人的典型遗址。北京地区在不晚于1万年前已经开始进入新石器时代。当时该地区人类定居生活固定化，逐渐从山洞中迁徙出来，到平原地区定居[12]。 ");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_cnn_lang_auto_impact_unicode(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Auto);
            builder.precision(Precision::Low);
            builder.keep_ponctuation(false);
            builder.lowercased(false);
            builder.default_stopwords(false);
            builder.unicode(true);

            let tokens = builder.build("距今60万年-2万年的时间内，北京地区处于旧石器时代，在周口店发现了旧石器时代早期北京直立人、中期新洞人和晚期山顶洞人的典型遗址。北京地区在不晚于1万年前已经开始进入新石器时代。当时该地区人类定居生活固定化，逐渐从山洞中迁徙出来，到平原地区定居[12]。 ");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_cnn_lang_auto_full_hight(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Auto);
            builder.precision(Precision::Hight);
            builder.keep_ponctuation(true);
            builder.lowercased(true);
            builder.default_stopwords(true);

            let tokens = builder.build("距今60万年-2万年的时间内，北京地区处于旧石器时代，在周口店发现了旧石器时代早期北京直立人、中期新洞人和晚期山顶洞人的典型遗址。北京地区在不晚于1万年前已经开始进入新石器时代。当时该地区人类定居生活固定化，逐渐从山洞中迁徙出来，到平原地区定居[12]。 ");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_cnn_lang_auto_full_low(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Auto);
            builder.precision(Precision::Low);
            builder.keep_ponctuation(true);
            builder.lowercased(true);
            builder.default_stopwords(true);

            let tokens = builder.build("距今60万年-2万年的时间内，北京地区处于旧石器时代，在周口店发现了旧石器时代早期北京直立人、中期新洞人和晚期山顶洞人的典型遗址。北京地区在不晚于1万年前已经开始进入新石器时代。当时该地区人类定居生活固定化，逐渐从山洞中迁徙出来，到平原地区定居[12]。 ");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_cnn_lang_cnn(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Forced(Lang::Fra));
            builder.precision(Precision::Low);
            builder.keep_ponctuation(false);
            builder.lowercased(false);
            builder.default_stopwords(false);
            builder.unicode(false);

            let tokens = builder.build("距今60万年-2万年的时间内，北京地区处于旧石器时代，在周口店发现了旧石器时代早期北京直立人、中期新洞人和晚期山顶洞人的典型遗址。北京地区在不晚于1万年前已经开始进入新石器时代。当时该地区人类定居生活固定化，逐渐从山洞中迁徙出来，到平原地区定居[12]。 ");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_cnn_lang_cnn_impact_ponctualtion(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Forced(Lang::Fra));
            builder.precision(Precision::Low);
            builder.keep_ponctuation(true);
            builder.lowercased(false);
            builder.default_stopwords(false);
            builder.unicode(false);

            let tokens = builder.build("距今60万年-2万年的时间内，北京地区处于旧石器时代，在周口店发现了旧石器时代早期北京直立人、中期新洞人和晚期山顶洞人的典型遗址。北京地区在不晚于1万年前已经开始进入新石器时代。当时该地区人类定居生活固定化，逐渐从山洞中迁徙出来，到平原地区定居[12]。 ");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_cnn_lang_cnn_impact_lowercased(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Forced(Lang::Fra));
            builder.precision(Precision::Low);
            builder.keep_ponctuation(false);
            builder.lowercased(true);
            builder.default_stopwords(false);
            builder.unicode(false);

            let tokens = builder.build("距今60万年-2万年的时间内，北京地区处于旧石器时代，在周口店发现了旧石器时代早期北京直立人、中期新洞人和晚期山顶洞人的典型遗址。北京地区在不晚于1万年前已经开始进入新石器时代。当时该地区人类定居生活固定化，逐渐从山洞中迁徙出来，到平原地区定居[12]。 ");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_cnn_lang_cnn_impact_unicode(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Forced(Lang::Fra));
            builder.precision(Precision::Low);
            builder.keep_ponctuation(false);
            builder.lowercased(false);
            builder.default_stopwords(false);
            builder.unicode(true);

            let tokens = builder.build("距今60万年-2万年的时间内，北京地区处于旧石器时代，在周口店发现了旧石器时代早期北京直立人、中期新洞人和晚期山顶洞人的典型遗址。北京地区在不晚于1万年前已经开始进入新石器时代。当时该地区人类定居生活固定化，逐渐从山洞中迁徙出来，到平原地区定居[12]。 ");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_cnn_lang_cnn_full_hight(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Forced(Lang::Fra));
            builder.precision(Precision::Hight);
            builder.keep_ponctuation(true);
            builder.lowercased(true);
            builder.default_stopwords(true);

            let tokens = builder.build("距今60万年-2万年的时间内，北京地区处于旧石器时代，在周口店发现了旧石器时代早期北京直立人、中期新洞人和晚期山顶洞人的典型遗址。北京地区在不晚于1万年前已经开始进入新石器时代。当时该地区人类定居生活固定化，逐渐从山洞中迁徙出来，到平原地区定居[12]。 ");
            for _ in tokens{
                continue
            }
        });
    }

    #[bench]
    fn bench_cnn_lang_cnn_full_low(b: &mut Bencher) {
        b.iter(|| {
            let mut builder = TokenizerBuilder::new();
            builder.lang_detection(LangDetection::Auto);
            builder.precision(Precision::Low);
            builder.keep_ponctuation(true);
            builder.lowercased(true);
            builder.default_stopwords(true);

            let tokens = builder.build("距今60万年-2万年的时间内，北京地区处于旧石器时代，在周口店发现了旧石器时代早期北京直立人、中期新洞人和晚期山顶洞人的典型遗址。北京地区在不晚于1万年前已经开始进入新石器时代。当时该地区人类定居生活固定化，逐渐从山洞中迁徙出来，到平原地区定居[12]。 ");
            for _ in tokens{
                continue
            }
        });
    }
}
