use once_cell::sync::Lazy;
use std::collections::HashSet;

/// Српски (Serbian) - Not Yet Implemented
pub static STOPWORDS_SRP: Lazy<HashSet<&'static str>> =
    Lazy::new(|| [""].iter().cloned().collect());
