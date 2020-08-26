use once_cell::sync::Lazy;
use std::collections::HashSet;

/// Hausa (Hausa)
pub static STOPWORDS_HAU: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    vec![
        "a", "amma", "ba", "ban", "ce", "cikin", "da", "don", "ga", "in", "ina", "ita", "ji", "ka",
        "ko", "kuma", "lokacin", "ma", "mai", "na", "ne", "ni", "sai", "shi", "su", "suka", "sun",
        "ta", "tafi", "take", "tana", "wani", "wannan", "wata", "ya", "yake", "yana", "yi", "za",
    ]
    .into_iter()
    .collect()
});
