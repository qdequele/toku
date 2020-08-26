use once_cell::sync::Lazy;
use std::collections::HashSet;

/// മലയാളം (Malayalam) - Not Yet Implemented
pub static STOPWORDS_MAL: Lazy<HashSet<&'static str>> =
    Lazy::new(|| vec![""].into_iter().collect());
