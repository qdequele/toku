use once_cell::sync::Lazy;
use std::collections::HashSet;

/// Igbo (Igbo) - Not Yet Implemented
pub static STOPWORDS_IBO: Lazy<HashSet<&'static str>> =
    Lazy::new(|| vec![""].into_iter().collect());
