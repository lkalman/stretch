use std::collections::HashSet as Set;

use crate::label::Label;

pub trait Tabular: std::marker::Sized
{
    type L: Label;
    fn new() -> Self;
    fn from_str( input: &'static str ) -> Result<Self, String>;
    /*
    fn new_from_elements( autosegments: &Vec<Self::L> ) -> Self;
    */
    // presupposes the same types of tabular instance,
    // same tiers, same elements:
    fn concatenate( &self, other: &Self ) -> Self;
    // temporarily public:
    fn abstractions( &self ) -> Set<Self>;
}

