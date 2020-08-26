use once_cell::sync::Lazy;
use std::collections::HashSet;

/// Eesti (Estonian)
pub static STOPWORDS_EST: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    vec![
        "aga", "ei", "et", "ja", "jah", "kas", "kui", "kõik", "ma", "me", "mida", "midagi", "mind",
        "minu", "mis", "mu", "mul", "mulle", "nad", "nii", "oled", "olen", "oli", "oma", "on",
        "pole", "sa", "seda", "see", "selle", "siin", "siis", "ta", "te", "ära",
    ]
    .into_iter()
    .collect()
});
