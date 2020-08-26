use once_cell::sync::Lazy;
use std::collections::HashSet;

/// سرائیکی (Saraiki) - Not Yet Implemented
pub static STOPWORDS_SKR: Lazy<HashSet<&'static str>> =
    Lazy::new(|| vec![""].into_iter().collect());
