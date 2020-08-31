use once_cell::sync::Lazy;
use std::collections::HashSet;

/// Türkmençe (Turkmen) - Not Yet Implemented
pub static STOPWORDS_TUK: Lazy<HashSet<&'static str>> = Lazy::new(HashSet::new);
