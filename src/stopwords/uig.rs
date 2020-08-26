use once_cell::sync::Lazy;
use std::collections::HashSet;

/// ئۇيغۇرچە (Uyghur) - Not Yet Implemented
pub static STOPWORDS_UIG: Lazy<HashSet<&'static str>> =
    Lazy::new(|| vec![""].into_iter().collect());
