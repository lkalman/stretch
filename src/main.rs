extern crate hun_grammar as gram;
extern crate tabular;

use crate::tabular::grapheme::GraphemeTabular;

fn main()
{
    let hun_gram: gram::hungarian_grapheme::HungarianTable =
        gram::hungarian_grapheme::HungarianTable::new();
    match
        gram::hungarian_grapheme::HungarianTable::from_string(
            &hun_gram, &"abdta" )
    {
        Ok( szo ) => { println!( "{}", &szo ); }
        Err( e ) => { println!( "Parse error: {}", &e ); }
    }
}
