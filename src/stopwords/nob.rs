use once_cell::sync::Lazy;
use std::collections::HashSet;

/// Bokmål (Bokmal) - Not Yet Implemented
pub static STOPWORDS_NOB: Lazy<HashSet<&'static str>> =
    Lazy::new(|| [""].iter().cloned().collect());
