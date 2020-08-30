use once_cell::sync::Lazy;
use std::collections::HashSet;

/// भोजपुरी (Bhojpuri) - Not Yet Implemented
pub static STOPWORDS_BHO: Lazy<HashSet<&'static str>> =
    Lazy::new(|| [""].iter().cloned().collect());
