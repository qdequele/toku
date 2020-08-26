use once_cell::sync::Lazy;
use std::collections::HashSet;

/// Malagasy (Malagasy) - Not Yet Implemented
pub static STOPWORDS_MLG: Lazy<HashSet<&'static str>> =
    Lazy::new(|| vec![""].into_iter().collect());
