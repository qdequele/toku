mod afr;
mod aka;
mod amh;
mod arb;
mod azj;
mod bel;
mod ben;
mod bho;
mod bul;
mod ceb;
mod ces;
mod cmn;
mod dan;
mod deu;
mod ell;
mod eng;
mod epo;
mod est;
mod fin;
mod fra;
mod guj;
mod hat;
mod hau;
mod heb;
mod hin;
mod hrv;
mod hun;
mod ibo;
mod ilo;
mod ind;
mod ita;
mod jav;
mod jpn;
mod kan;
mod kat;
mod khm;
mod kin;
mod kor;
mod kur;
mod lat;
mod lav;
mod lit;
mod mai;
mod mal;
mod mar;
mod mkd;
mod mlg;
mod mya;
mod nep;
mod nld;
mod nno;
mod nob;
mod nya;
mod ori;
mod orm;
mod pan;
mod pes;
mod pol;
mod por;
mod ron;
mod run;
mod rus;
mod sin;
mod skr;
mod slk;
mod slv;
mod sna;
mod som;
mod spa;
mod srp;
mod swe;
mod tam;
mod tel;
mod tgl;
mod tha;
mod tir;
mod tuk;
mod tur;
mod uig;
mod ukr;
mod urd;
mod uzb;
mod vie;
mod ydd;
mod yor;
mod zul;

pub use afr::STOPWORDS_AFR;
pub use aka::STOPWORDS_AKA;
pub use amh::STOPWORDS_AMH;
pub use arb::STOPWORDS_ARB;
pub use azj::STOPWORDS_AZJ;
pub use bel::STOPWORDS_BEL;
pub use ben::STOPWORDS_BEN;
pub use bho::STOPWORDS_BHO;
pub use bul::STOPWORDS_BUL;
pub use ceb::STOPWORDS_CEB;
pub use ces::STOPWORDS_CES;
pub use cmn::STOPWORDS_CMN;
pub use dan::STOPWORDS_DAN;
pub use deu::STOPWORDS_DEU;
pub use ell::STOPWORDS_ELL;
pub use eng::STOPWORDS_ENG;
pub use epo::STOPWORDS_EPO;
pub use est::STOPWORDS_EST;
pub use fin::STOPWORDS_FIN;
pub use fra::STOPWORDS_FRA;
pub use guj::STOPWORDS_GUJ;
pub use hat::STOPWORDS_HAT;
pub use hau::STOPWORDS_HAU;
pub use heb::STOPWORDS_HEB;
pub use hin::STOPWORDS_HIN;
pub use hrv::STOPWORDS_HRV;
pub use hun::STOPWORDS_HUN;
pub use ibo::STOPWORDS_IBO;
pub use ilo::STOPWORDS_ILO;
pub use ind::STOPWORDS_IND;
pub use ita::STOPWORDS_ITA;
pub use jav::STOPWORDS_JAV;
pub use jpn::STOPWORDS_JPN;
pub use kan::STOPWORDS_KAN;
pub use kat::STOPWORDS_KAT;
pub use khm::STOPWORDS_KHM;
pub use kin::STOPWORDS_KIN;
pub use kor::STOPWORDS_KOR;
pub use kur::STOPWORDS_KUR;
pub use lat::STOPWORDS_LAT;
pub use lav::STOPWORDS_LAV;
pub use lit::STOPWORDS_LIT;
pub use mai::STOPWORDS_MAI;
pub use mal::STOPWORDS_MAL;
pub use mar::STOPWORDS_MAR;
pub use mkd::STOPWORDS_MKD;
pub use mlg::STOPWORDS_MLG;
pub use mya::STOPWORDS_MYA;
pub use nep::STOPWORDS_NEP;
pub use nld::STOPWORDS_NLD;
pub use nno::STOPWORDS_NNO;
pub use nob::STOPWORDS_NOB;
pub use nya::STOPWORDS_NYA;
pub use ori::STOPWORDS_ORI;
pub use orm::STOPWORDS_ORM;
pub use pan::STOPWORDS_PAN;
pub use pes::STOPWORDS_PES;
pub use pol::STOPWORDS_POL;
pub use por::STOPWORDS_POR;
pub use ron::STOPWORDS_RON;
pub use run::STOPWORDS_RUN;
pub use rus::STOPWORDS_RUS;
pub use sin::STOPWORDS_SIN;
pub use skr::STOPWORDS_SKR;
pub use slk::STOPWORDS_SLK;
pub use slv::STOPWORDS_SLV;
pub use sna::STOPWORDS_SNA;
pub use som::STOPWORDS_SOM;
pub use spa::STOPWORDS_SPA;
pub use srp::STOPWORDS_SRP;
pub use swe::STOPWORDS_SWE;
pub use tam::STOPWORDS_TAM;
pub use tel::STOPWORDS_TEL;
pub use tgl::STOPWORDS_TGL;
pub use tha::STOPWORDS_THA;
pub use tir::STOPWORDS_TIR;
pub use tuk::STOPWORDS_TUK;
pub use tur::STOPWORDS_TUR;
pub use uig::STOPWORDS_UIG;
pub use ukr::STOPWORDS_UKR;
pub use urd::STOPWORDS_URD;
pub use uzb::STOPWORDS_UZB;
pub use vie::STOPWORDS_VIE;
pub use ydd::STOPWORDS_YDD;
pub use yor::STOPWORDS_YOR;
pub use zul::STOPWORDS_ZUL;

use std::collections::HashSet;
use whatlang::{Lang, Script};

pub struct StopWords;

impl StopWords {
    pub fn match_lang(word: &str, lang: Option<Lang>) -> bool {
        if let Some(lang) = lang {
            if Self::for_lang(lang).contains(word) {
                return true;
            }
        }
        false
    }

    pub fn detect_lang(text: &str, script: Script) -> Option<Lang> {
        let script_langs = script_langs(script);

        let mut count = 0;
        let mut lang = None;

        let text_split = text.split_whitespace().collect::<Vec<&str>>();

        for script_lang in script_langs {
            let lang_stopwords = Self::for_lang(*script_lang);

            if !lang_stopwords.is_empty() {
                let mut lang_count = 0;

                for word in &text_split {
                    if lang_stopwords.contains(word) {
                        lang_count += 1;
                    }
                }

                if lang_count > 0 {
                    if lang_count > count {
                        count = lang_count;
                        lang = Some(*script_lang);
                    }
                }
            }
        }

        lang
    }

    pub fn for_lang(lang: Lang) -> &'static HashSet<&'static str> {
        match lang {
            Lang::Afr => &*STOPWORDS_AFR,
            Lang::Aka => &*STOPWORDS_AKA,
            Lang::Amh => &*STOPWORDS_AMH,
            Lang::Arb => &*STOPWORDS_ARB,
            Lang::Azj => &*STOPWORDS_AZJ,
            Lang::Bel => &*STOPWORDS_BEL,
            Lang::Ben => &*STOPWORDS_BEN,
            Lang::Bho => &*STOPWORDS_BHO,
            Lang::Bul => &*STOPWORDS_BUL,
            Lang::Ceb => &*STOPWORDS_CEB,
            Lang::Ces => &*STOPWORDS_CES,
            Lang::Cmn => &*STOPWORDS_CMN,
            Lang::Dan => &*STOPWORDS_DAN,
            Lang::Deu => &*STOPWORDS_DEU,
            Lang::Ell => &*STOPWORDS_ELL,
            Lang::Eng => &*STOPWORDS_ENG,
            Lang::Epo => &*STOPWORDS_EPO,
            Lang::Est => &*STOPWORDS_EST,
            Lang::Fin => &*STOPWORDS_FIN,
            Lang::Fra => &*STOPWORDS_FRA,
            Lang::Guj => &*STOPWORDS_GUJ,
            Lang::Hat => &*STOPWORDS_HAT,
            Lang::Hau => &*STOPWORDS_HAU,
            Lang::Heb => &*STOPWORDS_HEB,
            Lang::Hin => &*STOPWORDS_HIN,
            Lang::Hrv => &*STOPWORDS_HRV,
            Lang::Hun => &*STOPWORDS_HUN,
            Lang::Ibo => &*STOPWORDS_IBO,
            Lang::Ilo => &*STOPWORDS_ILO,
            Lang::Ind => &*STOPWORDS_IND,
            Lang::Ita => &*STOPWORDS_ITA,
            Lang::Jav => &*STOPWORDS_JAV,
            Lang::Jpn => &*STOPWORDS_JPN,
            Lang::Kan => &*STOPWORDS_KAN,
            Lang::Kat => &*STOPWORDS_KAT,
            Lang::Khm => &*STOPWORDS_KHM,
            Lang::Kin => &*STOPWORDS_KIN,
            Lang::Kor => &*STOPWORDS_KOR,
            Lang::Kur => &*STOPWORDS_KUR,
            Lang::Lat => &*STOPWORDS_LAT,
            Lang::Lav => &*STOPWORDS_LAV,
            Lang::Lit => &*STOPWORDS_LIT,
            Lang::Mai => &*STOPWORDS_MAI,
            Lang::Mal => &*STOPWORDS_MAL,
            Lang::Mar => &*STOPWORDS_MAR,
            Lang::Mkd => &*STOPWORDS_MKD,
            Lang::Mlg => &*STOPWORDS_MLG,
            Lang::Mya => &*STOPWORDS_MYA,
            Lang::Nep => &*STOPWORDS_NEP,
            Lang::Nld => &*STOPWORDS_NLD,
            Lang::Nno => &*STOPWORDS_NNO,
            Lang::Nob => &*STOPWORDS_NOB,
            Lang::Nya => &*STOPWORDS_NYA,
            Lang::Ori => &*STOPWORDS_ORI,
            Lang::Orm => &*STOPWORDS_ORM,
            Lang::Pan => &*STOPWORDS_PAN,
            Lang::Pes => &*STOPWORDS_PES,
            Lang::Pol => &*STOPWORDS_POL,
            Lang::Por => &*STOPWORDS_POR,
            Lang::Ron => &*STOPWORDS_RON,
            Lang::Run => &*STOPWORDS_RUN,
            Lang::Rus => &*STOPWORDS_RUS,
            Lang::Sin => &*STOPWORDS_SIN,
            Lang::Skr => &*STOPWORDS_SKR,
            Lang::Slk => &*STOPWORDS_SLK,
            Lang::Slv => &*STOPWORDS_SLV,
            Lang::Sna => &*STOPWORDS_SNA,
            Lang::Som => &*STOPWORDS_SOM,
            Lang::Spa => &*STOPWORDS_SPA,
            Lang::Srp => &*STOPWORDS_SRP,
            Lang::Swe => &*STOPWORDS_SWE,
            Lang::Tam => &*STOPWORDS_TAM,
            Lang::Tel => &*STOPWORDS_TEL,
            Lang::Tgl => &*STOPWORDS_TGL,
            Lang::Tha => &*STOPWORDS_THA,
            Lang::Tir => &*STOPWORDS_TIR,
            Lang::Tuk => &*STOPWORDS_TUK,
            Lang::Tur => &*STOPWORDS_TUR,
            Lang::Uig => &*STOPWORDS_UIG,
            Lang::Ukr => &*STOPWORDS_UKR,
            Lang::Urd => &*STOPWORDS_URD,
            Lang::Uzb => &*STOPWORDS_UZB,
            Lang::Vie => &*STOPWORDS_VIE,
            Lang::Ydd => &*STOPWORDS_YDD,
            Lang::Yor => &*STOPWORDS_YOR,
            Lang::Zul => &*STOPWORDS_ZUL,
        }
    }
}

fn script_langs(script: Script) -> &'static [Lang] {
    match script {
        Script::Latin => &[
            Lang::Spa,
            Lang::Eng,
            Lang::Por,
            Lang::Ind,
            Lang::Fra,
            Lang::Deu,
            Lang::Jav,
            Lang::Vie,
            Lang::Ita,
            Lang::Tur,
            Lang::Pol,
            Lang::Orm,
            Lang::Ron,
            Lang::Hau,
            Lang::Hrv,
            Lang::Nld,
            Lang::Kur,
            Lang::Yor,
            Lang::Uzb,
            Lang::Ibo,
            Lang::Ceb,
            Lang::Tgl,
            Lang::Hun,
            Lang::Azj,
            Lang::Ces,
            Lang::Mlg,
            Lang::Nya,
            Lang::Kin,
            Lang::Zul,
            Lang::Swe,
            Lang::Som,
            Lang::Ilo,
            Lang::Uig,
            Lang::Hat,
            Lang::Aka,
            Lang::Sna,
            Lang::Afr,
            Lang::Fin,
            Lang::Run,
            Lang::Tuk,
            Lang::Dan,
            Lang::Nob,
            Lang::Nno,
            Lang::Lit,
            Lang::Slv,
            Lang::Epo,
            Lang::Lav,
            Lang::Est,
            Lang::Lat,
            Lang::Slk,
        ],
        Script::Cyrillic => &[
            Lang::Rus,
            Lang::Ukr,
            Lang::Srp,
            Lang::Azj,
            Lang::Bel,
            Lang::Bul,
            Lang::Tuk,
            Lang::Mkd,
        ],
        Script::Arabic => &[Lang::Arb, Lang::Urd, Lang::Skr, Lang::Uig, Lang::Pes],
        Script::Devanagari => &[Lang::Hin, Lang::Mar, Lang::Mai, Lang::Bho, Lang::Nep],
        Script::Ethiopic => &[Lang::Amh, Lang::Tir],
        Script::Hebrew => &[Lang::Heb, Lang::Ydd],
        Script::Mandarin => &[Lang::Cmn],
        Script::Bengali => &[Lang::Ben],
        Script::Hangul => &[Lang::Kor],
        Script::Georgian => &[Lang::Kat],
        Script::Greek => &[Lang::Ell],
        Script::Kannada => &[Lang::Kan],
        Script::Tamil => &[Lang::Tam],
        Script::Thai => &[Lang::Tha],
        Script::Gujarati => &[Lang::Guj],
        Script::Gurmukhi => &[Lang::Pan],
        Script::Telugu => &[Lang::Tel],
        Script::Malayalam => &[Lang::Mal],
        Script::Oriya => &[Lang::Ori],
        Script::Myanmar => &[Lang::Mya],
        Script::Sinhala => &[Lang::Sin],
        Script::Khmer => &[Lang::Khm],
        Script::Katakana | Script::Hiragana => &[Lang::Jpn],
    }
}
