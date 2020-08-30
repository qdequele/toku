use once_cell::sync::Lazy;
use std::collections::HashSet;

/// Akan (Akan) - Not Yet Implemented
pub static STOPWORDS_AKA: Lazy<HashSet<&'static str>> =
    Lazy::new(|| [""].iter().cloned().collect());
