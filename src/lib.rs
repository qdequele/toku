mod stopwords;
mod tokenizer;

pub use stopwords::*;
pub use tokenizer::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
