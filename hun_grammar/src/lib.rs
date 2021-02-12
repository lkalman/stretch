extern crate tabular;
extern crate peg;
pub mod hungarian_grapheme;

mod convert_graphemes;
mod simple_grapheme;
mod obstruent_voice;
mod orthographic_combinations;
mod combine_identical;
mod simplify_geminates;

// convert_graphemes::convert_graphemes<'a>( s: &'a str ) ->
//     Result<Vec<&'a str>>, String>;
