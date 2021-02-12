extern crate peg;
use crate::simple_grapheme;
use crate::obstruent_voice;
use crate::orthographic_combinations;
use crate::combine_identical;
use crate::simplify_geminates;

// Convert to single-grapheme sequences;
// apply obstruent voice assimilation;
// convert orthographical combinations into geminates
// simplify geminate consonants next to consonants

pub fn convert_graphemes( s: &'static str )
    -> Result<Vec<String>, String>
{
    match simple_grapheme::simple_grapheme::input( s )
    {
        Ok( graphemic ) =>
        {
            let mut combined =
                orthographic_combinations::orthographic_combinations::input(
                    &graphemic ).unwrap();

            let mut modified = true;
            while modified
            {
                let new_assimilated =
                    obstruent_voice::obstruent_voice::input( &combined ).
                    unwrap();
                if new_assimilated == combined
                {
                    modified = false;
                }
                else { combined = new_assimilated; }
            }

            combined =
                combine_identical::combine_identical::input( &combined ).
                unwrap();

            let mut cons = false;
            Ok(
                simplify_geminates::simplify_geminates::input(
                    &combined, &mut cons ).unwrap() )
        }
        Err( e ) => Err( format!( "simple_grapheme: {}", e.to_string() ) )
    }
}
