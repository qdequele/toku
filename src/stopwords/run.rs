use once_cell::sync::Lazy;
use std::collections::HashSet;

/// Ikirundi (Rundi) - Not Yet Implemented
pub static STOPWORDS_RUN: Lazy<HashSet<&'static str>> =
    Lazy::new(|| vec![""].into_iter().collect());
