extern crate peg;

pub mod tabular;
pub mod grapheme;
pub mod table;
mod table_grammar;
mod label;
mod grammar;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
