#![cfg_attr(feature = "nightly", feature(test))]

mod stopwords;
mod tokenizer;

pub use stopwords::*;
pub use tokenizer::*;

use whatlang::{Lang, Script};

fn script_to_langs(script: Script) -> &'static [Lang] {
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

pub fn lang_to_script(lang: Lang) -> Script {
    match lang {
        Lang::Spa
        | Lang::Eng
        | Lang::Por
        | Lang::Ind
        | Lang::Fra
        | Lang::Deu
        | Lang::Jav
        | Lang::Vie
        | Lang::Ita
        | Lang::Tur
        | Lang::Pol
        | Lang::Orm
        | Lang::Ron
        | Lang::Hau
        | Lang::Hrv
        | Lang::Nld
        | Lang::Kur
        | Lang::Yor
        | Lang::Uzb
        | Lang::Ibo
        | Lang::Ceb
        | Lang::Tgl
        | Lang::Hun
        | Lang::Ces
        | Lang::Mlg
        | Lang::Nya
        | Lang::Kin
        | Lang::Zul
        | Lang::Swe
        | Lang::Som
        | Lang::Ilo
        | Lang::Hat
        | Lang::Aka
        | Lang::Sna
        | Lang::Afr
        | Lang::Fin
        | Lang::Run
        | Lang::Dan
        | Lang::Nob
        | Lang::Nno
        | Lang::Lit
        | Lang::Slv
        | Lang::Epo
        | Lang::Lav
        | Lang::Est
        | Lang::Lat
        | Lang::Slk => Script::Latin,
        Lang::Rus
        | Lang::Ukr
        | Lang::Srp
        | Lang::Azj
        | Lang::Bel
        | Lang::Bul
        | Lang::Tuk
        | Lang::Mkd => Script::Cyrillic,
        Lang::Arb | Lang::Urd | Lang::Skr | Lang::Uig | Lang::Pes => Script::Arabic,
        Lang::Hin | Lang::Mar | Lang::Mai | Lang::Bho | Lang::Nep => Script::Devanagari,
        Lang::Amh | Lang::Tir => Script::Ethiopic,
        Lang::Heb | Lang::Ydd => Script::Hebrew,
        Lang::Cmn => Script::Mandarin,
        Lang::Ben => Script::Bengali,
        Lang::Kor => Script::Hangul,
        Lang::Kat => Script::Georgian,
        Lang::Ell => Script::Greek,
        Lang::Kan => Script::Kannada,
        Lang::Tam => Script::Tamil,
        Lang::Tha => Script::Thai,
        Lang::Guj => Script::Gujarati,
        Lang::Pan => Script::Gurmukhi,
        Lang::Tel => Script::Telugu,
        Lang::Mal => Script::Malayalam,
        Lang::Ori => Script::Oriya,
        Lang::Mya => Script::Myanmar,
        Lang::Sin => Script::Sinhala,
        Lang::Khm => Script::Khmer,
        Lang::Jpn => Script::Katakana,
    }
}
