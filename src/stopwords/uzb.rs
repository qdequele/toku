use once_cell::sync::Lazy;
use std::collections::HashSet;

/// Oʻzbekcha (Uzbek) - Not Yet Implemented
pub static STOPWORDS_UZB: Lazy<HashSet<&'static str>> =
    Lazy::new(|| vec![""].into_iter().collect());
