use once_cell::sync::Lazy;
use std::collections::HashSet;

/// తెలుగు (Telugu) - Not Yet Implemented
pub static STOPWORDS_TEL: Lazy<HashSet<&'static str>> =
    Lazy::new(|| [""].iter().cloned().collect());
