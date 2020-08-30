use once_cell::sync::Lazy;
use std::collections::HashSet;

/// मैथिली (Maithili) - Not Yet Implemented
pub static STOPWORDS_MAI: Lazy<HashSet<&'static str>> =
    Lazy::new(|| [""].iter().cloned().collect());
