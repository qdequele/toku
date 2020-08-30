use once_cell::sync::Lazy;
use std::collections::HashSet;

/// ଓଡ଼ିଆ (Oriya) - Not Yet Implemented
pub static STOPWORDS_ORI: Lazy<HashSet<&'static str>> =
    Lazy::new(|| [""].iter().cloned().collect());
