use once_cell::sync::Lazy;
use std::collections::HashSet;

/// ქართული (Georgian) - Not Yet Implemented
pub static STOPWORDS_KAT: Lazy<HashSet<&'static str>> =
    Lazy::new(|| [""].iter().cloned().collect());
