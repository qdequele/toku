use once_cell::sync::Lazy;
use std::collections::HashSet;

/// Беларуская (Belarusian) - Not Yet Implemented
pub static STOPWORDS_BEL: Lazy<HashSet<&'static str>> = Lazy::new(HashSet::new);
