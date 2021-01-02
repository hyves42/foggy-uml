pub mod datatypes;
pub mod parsers;
pub mod builders;
pub mod parseutils;
pub mod preprocessor;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
