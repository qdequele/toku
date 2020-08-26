use once_cell::sync::Lazy;
use std::collections::HashSet;

/// አማርኛ (Amharic) - Not Yet Implemented
pub static STOPWORDS_AMH: Lazy<HashSet<&'static str>> =
    Lazy::new(|| vec![""].into_iter().collect());
