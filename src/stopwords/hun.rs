use once_cell::sync::Lazy;
use std::collections::HashSet;

/// Magyar (Hungarian)
pub static STOPWORDS_HUN: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        "a", "át", "az", "be", "csak", "de", "egy", "el", "én", "és", "fel", "hát", "hogy", "ide",
        "igen", "ki", "le", "lesz", "meg", "mi", "mint", "nem", "õ", "oda", "õk", "ön", "össze",
        "rá", "szét", "te", "ti", "vagy", "van", "vissza", "volt",
    ]
    .iter()
    .cloned()
    .collect()
});
