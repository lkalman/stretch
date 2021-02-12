// use std::io;
// use std::io::{Read, BufReader};
// use std::fs::File;

use std::collections::HashSet as Set;
use std::iter::FromIterator;

use crate::tabular::grapheme;
use crate::tabular::tabular::Tabular;

use crate::tabular::table;
use table::Table;

/// convert_graphemes::convert_graphemes<'a>( s: &'a str ) ->
///     Result<String, String>;
use crate::convert_graphemes;

/**!
A converter from Hungarian grapheme sequences,
implements the `tabular::grapheme::GraphemeTabular` trait,
with output type being `tabular::Table`.
*/
pub struct HungarianTable
{
    number_of_tiers: usize,
    tier_names: Vec<&'static str>,
    autosegments: Vec<&'static str>,
    no_ocp: Set<usize>,         // indices into `tier_names`
}

/// Implementation of a converter trait
impl grapheme::GraphemeTabular<Table<'static>> for HungarianTable
{
    type TierName = &'static str;

    fn new() -> Self
    {
        HungarianTable
        {
            number_of_tiers: 7,
            tier_names: vec![
                "tongue", "jaw", "lips", "voice", "airflow",
                "time", "eventuality"],
            autosegments: vec![
   // Tongue:
        "neutral",              // also Jaw and Lips
        "front",
        "back",
        "dentialveolar",
        "coronal",
        "palatal",
        "velar",
        // Jaw:
        "extra-low",
        "low",
        "mid",
        "mid-high",
        "high",
        // Lips:
        "rounded",
        "unrounded",
        "closed",
        "approximate",
        "lower-to-teeth",
        // Voice:
        "voiced",
        "voiceless",
        // Airflow:
        "open",
        "slit",
        "stop",
        "lateral",
        "nasal",
        "approximate",
        "trill",
        // Time:
        "X",
        // Eventuality:
        "state",
        "transition",
        "event"
            ],
            no_ocp: Set::from_iter( vec![5, 6] ),    // time
        }
    }
    
    fn tier_name_to_tier_index(
        &self, tier_name: &Self::TierName )
        -> usize
    {
        return <[_]>::binary_search( &self.tier_names, &tier_name ).ok().unwrap()
    }
    fn tier_index_to_tier_name(
        &self, tier_index: usize )
        -> Self::TierName
    {
        return self.tier_names[tier_index]
    }
    fn from_string( &self, graphemes: &'static str )
                         -> Result<Table<'static>, String>
    {
        match convert_graphemes::convert_graphemes( graphemes )
        {
            Ok( gr ) =>
            {
                let mut result;
                match HungarianTable::from_grapheme( &gr[0] )
                {
                    Ok( tab ) => { result = tab; }
                    Err( e ) => { return Err( e ); }
                }
                for i in 1..<[_]>::len( &gr )
                {
                    match HungarianTable::from_grapheme( &gr[i] )
                    {
                        Ok( mut tab ) =>
                        {
                            result =
                                Tabular::concatenate(
                                    &result,
                                    &mut tab );
                        }
                        Err( e ) =>
                        {
                            return Err( e );
                        }
                    }
                }
                Ok( result )
            }
            Err( e ) =>
            {
                Err( format!( "Parse error: {}", &e ) )
                // std::process::exit( 1 );
            }
        }
    }
}

/// Traitless methods of a converter structure:
impl HungarianTable
{
    pub fn from_grapheme<'a>( grapheme: &'a str ) ->
        Result<Table<'static>, String>
    {
        match grapheme
        {
            // vowels
            "a" =>
                Table::from_str(
                    "back, low, rounded, voiced, open, X, state;" ),
            "e" =>
                Table::from_str(
                    "front, low, unrounded, voiced, open, X, state;" ),
            "i" =>
                Table::from_str(
                    "front, high, unrounded, voiced, open, X, state;" ),
            "o" =>
                Table::from_str(
                    "back, mid, rounded, voiced, open, X, state;" ),
            "u" =>
                Table::from_str(
                    "back, high, rounded, voiced, open, X, state;" ),
            "ö" =>
                Table::from_str(
                    "front, mid, rounded, voiced, open, X, state;" ),
            "ü" =>
                Table::from_str(
                    "front, mid, rounded, voiced, open, X, state;" ),
            // long vowels
            "á" =>
                Table::from_str(
                    "2 back, 2 extra-low, 2 unrounded, 2 voiced, 2 open, X X, 2 state" ),
            "é" =>
                Table::from_str(
                    "2 front, 2 mid-high, 2 unrounded, 2 voiced, 2 open, X X, 2 state" ),
            "í" =>
                Table::from_str(
                    "2 front, 2 high, 2 unrounded, 2 voiced, 2 open, X X, 2 state" ),
            "ó" =>
                Table::from_str(
                    "2 back, 2 mid-high, 2 rounded, 2 voiced, 2 open, X X, 2 state;" ),
            "ú" =>
                Table::from_str(
                    "2 back, 2 high, 2 rounded, 2 voiced, 2 open, X X, 2 state" ),
            "ő" =>
                Table::from_str(
                    "2 front, 2 mid-high, 2 rounded, 2 voiced, 2 open, X X, 2 state;" ),
            "ű" =>
                Table::from_str(
                    "2 front, 2 high, 2 rounded, 2 voiced, 2 open, X X, 2 state;" ),
            // stops
            "b" =>
                Table::from_str(
                    "2 _, 2 _, 2 lips, 2 voiced, 2 stop, 2 X, state event;" ),
            "d" =>
                Table::from_str(
                    "2 dentialveolar, 2 _, 2 _, 2 voiced, 2 stop, 2 X, state event;" ),
            "g" =>
                Table::from_str(
                    "2 velar, 2 _, 2 _, 2 voiced, 2 stop, 2 X, state event;" ),
            "p" =>
                Table::from_str(
                    "2 _, 2 _, 2 lips, 2 voiceless, 2 stop, 2 X, state event;" ),
            "t" =>
                Table::from_str(
                    "2 dentialveolar, 2 _, 2 _, 2 voiceless, 2 stop, 2 X, state event;" ),
            "c" =>
                Table::from_str(
                    "2 dentialveolar, 2 _, 2 _, 2 voiceless, 2 stop, 2 X, state event;" ),
            "č" =>
                Table::from_str(
                    "2 coronal, 2 _, 2 _, 2 voiceless, 2 stop, 2 X, state event;" ),
            "ď" =>
                Table::from_str(
                    "2 palatal, 2 _, 2 _, 2 voiced, 2 stop, 2 X, state event;" ),
            "ť" =>
                Table::from_str(
                    "2 palatal, 2 _, 2 _, 2 voiceless, 2 stop, 2 X, state event;" ),
            "k" =>
                Table::from_str(
                    "2 velar, 2 _, 2 _, 2 voiceless, 2 stop, 2 X, state event;" ),
            // geminate stops
            "B" =>
                Table::from_str(
                    "3 _, 3 _, 3 lips, 3 voiced, 3 stop, X 2 X, 2 state event;" ),
            "D" =>
                Table::from_str(
                    "3 dentialveolar, 3 _, 3 _, 3 voiced, 3 stop , X 2 X, 2 state event;" ),
            "C" =>
                Table::from_str(
                    "3 dentialveolar, 3 _, 3 _, 3 voiceless, 3 stop, X 2 X, 2 state event;" ),
            "Č" =>
                Table::from_str(
                    "3 coronal, 3 _, 3 _, 3 voiceless, 3 stop, X 2 X, 2 state event;" ),
            "Ď" =>
                Table::from_str(
                    "3 palatal, 3 _, 3 _, 3 voiced, 3 stop, X 2 X, 2 state event;" ),
            "Ť" =>
                Table::from_str(
                    "3 palatal, 3 _, 3 _, 3 voiceless, 3 stop, X 2 X, 2 state event;" ),
            "G" =>
                Table::from_str(
                    "3 velar, 3 _, 3 _, 3 voiced, 3 stop, X 2 X, 2 state event;" ),
            "P" =>
                Table::from_str(
                    "3 _, 3 _, 3 lips, 3 voiceless, 3 stop, X 2 X, 2 state event;" ),
            "T" =>
                Table::from_str(
                    "3 dentialveolar, 3 _, 3 _, 3 voiceless, 3 stop, X 2 X, 2 state event;" ),
            "K" =>
                Table::from_str(
                    "3 velar, 3 _, 3 _, 3 voiceless, 3 stop, X 2 X, 2 state event;" ),
            // fricatives
            "v" =>
                Table::from_str(
                    "_, _, lower-to-teeth, voiced, approximate, X, state;" ),
            "f" =>
                Table::from_str(
                    "_, _, lower-to-teeth, voiceless, slit, X, state;" ),
            "z" =>
                Table::from_str(
                    "dentialveolar, _, _, voiced, slit, X, state;" ),
            "s" =>
                Table::from_str(
                    "dentialveolar, _, _, voiceless, slit, X, state;" ),
            "ž" =>
                Table::from_str(
                    "coronal, _, _, voiced, slit, X, state;" ),
            "š" =>
                Table::from_str(
                    "coronal, _, _, voiceless, slit, X, state;" ),
            // geminate fricatives
            "V" =>
                Table::from_str(
                    "2 _, 2 _, 2 lower-to-teeth, 2 voiced, 2 approximate, X X, 2 state;" ),
            "F" =>
                Table::from_str(
                    "2 _, 2 _, 2 lower-to-teeth, 2 voiceless, 2 slit, X X, 2 state;" ),
            "Z" =>
                Table::from_str(
                    "2 dentialveolar, 2 _, 2 _, 2 voiced, 2 slit, X X, 2 state;" ),
            "S" =>
                Table::from_str(
                    "2 dentialveolar, 2 _, 2 _, 2 voiceless, 2 slit, X X, 2 state;" ),
            "Ž" =>
                Table::from_str(
                    "2 coronal, 2 _, 2 _, 2 voiced, 2 slit, X X, 2 state;" ),
            "Š" =>
                Table::from_str(
                    "2 coronal, 2 _, 2 _, 2 voiceless, 2 slit, X X, 2 event;" ),
            // approximants
            "j" =>
                Table::from_str(
                    "palatal, _, _, voiced, approximate, X, state;" ),
            "h" =>
                Table::from_str(
                    "_, _, _, voiceless, approximate, X, state" ),
            "l" =>
                Table::from_str(
                    "dentialveolar, _, _, voiced, lateral, X, state;" ),
            // geminate approximants
            "H" =>
                Table::from_str(
                    "2 _, 2 _, 2 _, 2 voiceless, 2 approximate, X X, 2 state" ),
            "J" =>
                Table::from_str(
                    "2 palatal, 2 _, 2 _, 2 voiced, 2 approximate, X X, 2 state;" ),
            "L" =>
                Table::from_str(
                    "2 dentialveolar, 2 _, 2 _, 2 voiced, 2 lateral, X X, 2 state;" ),
            // nasals
            "m" =>
                Table::from_str(
                    "_, _, closed, voiced, nasal, X, state;" ),
            "n" =>
                Table::from_str(
                    "dentialveolar, _, _, voiced, nasal, X, state;" ),
            "ń" =>
                Table::from_str(
                    "2 palatal, 2 _, 2 _, 2 voiced, nasal slit, 2 X, 2 state;" ),
            // geminate nasals
            "M" =>
                Table::from_str(
                    "2 _, 2 _, 2 closed, 2 voiced, 2 nasal, X X, 2 state;" ),
            "N" =>
                Table::from_str(
                    "2 dentialveolar, 2 _, 2 _, 2 voiced, 2 nasal, X X, 2 state;" ),
            "Ń" =>
                Table::from_str(
                    "2 palatal, 2 _, 2 _, 2 voiced, 2 nasal, X X, 2 state;" ),
            // trill
            "r" =>
                Table::from_str(
                    "dentialveolar, _, _, voiced, trill, X, state;" ),
            // geminate trill
            "R" =>
                Table::from_str(
                    "2 dentialveolar, 2 _, 2 _, 2 voiced, 2 trill, X X, 2 state;" ),
            _ =>
                Err( format!( "Bad grapheme: \"{}\"", grapheme ) )
        }
    }
}
