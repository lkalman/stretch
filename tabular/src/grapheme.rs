use crate::tabular::Tabular;

/**!
A converter of grapheme sequences to tabular representation
in a given language must implement this trait.
*/
pub trait GraphemeTabular<Tab: Tabular>
{
    type TierName;

    fn new() -> Self;
    fn tier_name_to_tier_index(
        &self, tier_name: &Self::TierName )
        -> usize;
    fn tier_index_to_tier_name(
        &self, tier_index: usize )
        -> Self::TierName;
    fn from_string( &self, graphemes: &'static str )
                     -> Result<Tab, String>;
}

