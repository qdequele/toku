use once_cell::sync::Lazy;
use std::collections::HashSet;

/// Kreyòl ayisyen (Haitian Creole) - Not Yet Implemented
pub static STOPWORDS_HAT: Lazy<HashSet<&'static str>> =
    Lazy::new(|| vec![""].into_iter().collect());
