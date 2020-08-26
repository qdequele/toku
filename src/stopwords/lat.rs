use once_cell::sync::Lazy;
use std::collections::HashSet;

/// Lingua Latina (Latin)
pub static STOPWORDS_LAT: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    vec![
        "a", "ab", "ac", "ad", "at", "atque", "aut", "autem", "cum", "de", "dum", "e", "erant",
        "erat", "est", "et", "etiam", "ex", "haec", "hic", "hoc", "in", "ita", "me", "nec",
        "neque", "non", "per", "qua", "quae", "quam", "qui", "quibus", "quidem", "quo", "quod",
        "re", "rebus", "rem", "res", "sed", "si", "sic", "sunt", "tamen", "tandem", "te", "ut",
        "vel",
    ]
    .into_iter()
    .collect()
});
