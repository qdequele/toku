use once_cell::sync::Lazy;
use std::collections::HashSet;

/// Chichewa (Chewa) - Not Yet Implemented
pub static STOPWORDS_NYA: Lazy<HashSet<&'static str>> =
    Lazy::new(|| [""].iter().cloned().collect());
