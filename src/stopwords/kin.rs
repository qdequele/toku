use once_cell::sync::Lazy;
use std::collections::HashSet;

/// Kinyarwanda (Kinyarwanda) - Not Yet Implemented
pub static STOPWORDS_KIN: Lazy<HashSet<&'static str>> =
    Lazy::new(|| vec![""].into_iter().collect());
