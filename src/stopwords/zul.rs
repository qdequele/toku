use once_cell::sync::Lazy;
use std::collections::HashSet;

/// IsiZulu (Zulu)
pub static STOPWORDS_ZUL: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    vec![
        "futhi",
        "kahle",
        "kakhulu",
        "kanye",
        "khona",
        "kodwa",
        "kungani",
        "kusho",
        "la",
        "lakhe",
        "lapho",
        "mina",
        "ngesikhathi",
        "nje",
        "phansi",
        "phezulu",
        "u",
        "ukuba",
        "ukuthi",
        "ukuze",
        "uma",
        "wahamba",
        "wakhe",
        "wami",
        "wase",
        "wathi",
        "yakhe",
        "zakhe",
        "zonke",
    ]
    .into_iter()
    .collect()
});
