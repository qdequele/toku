use once_cell::sync::Lazy;
use std::collections::HashSet;

/// Oromoo (Oromo) - Not Yet Implemented
pub static STOPWORDS_ORM: Lazy<HashSet<&'static str>> =
    Lazy::new(|| [""].iter().cloned().collect());
