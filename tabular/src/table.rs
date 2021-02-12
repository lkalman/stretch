use std::fmt;
use std::collections::HashSet as Set;
use std::ops::{Index,IndexMut};

use crate::tabular;
use crate::label::Label;
use crate::table_grammar;

// Delta (Hertz 1990) is an autosegmental representation
// which does not have OCP built in, and floating segments
// cannot be distinguished.

// It is a tabular representation, where each tier (“stream”)
// consists of empty or filled slots delimited by boundaries,
// and the boundaries can be synchronized across tiers.

// In this representation, boundaries will not be represented
// explicitly, a stream consists of a sequence of slots,
// and each slot contains information on how many boundaries
// it spans on other tiers.

impl<'a> Label for &'a str
{
}


#[derive( PartialEq, Eq, std::hash::Hash, Clone, PartialOrd, Ord, Debug, Copy )]
// temporarily public:
pub enum Element<'a>
{
    Null,
    Element( &'a str )
}

impl<'a> fmt::Display for Element<'a>
{
    fn fmt( &self, s: &mut fmt::Formatter ) -> fmt::Result
    {
        match self
        {
            Element::Element( l ) => write!( s, "{}", l ),
            _ => write!( s, "&nbsp;" )
        }
    }
}

#[derive( PartialEq, Eq, std::hash::Hash, Clone, PartialOrd, Ord, Debug )]
// temporarily public (with fields):
pub struct Slot<'a>
{
    element: Element<'a>,
    left_aligned: bool,
    right_aligned: bool,
}

impl<'a> Slot<'a>
{
    pub fn new( element: Element<'a> ) -> Self
    {
        Slot { element: element,
               left_aligned: false,
               right_aligned: false, }
    }

    pub fn equal_element( &self, el: &'a str ) -> bool
    {
        match self.element
        {
            Element::Element( some_el ) => el == some_el,
            _ => false
        }
    }

    pub fn set_element( &mut self, el: &'a str )
    {
        self.element = Element::Element( el );
    }

    pub fn is_gap( &self ) -> bool
    {
        self.element == Element::Null
    }

    pub fn set_gap( &mut self )
    {
        self.element = Element::Null;
    }

    pub fn is_left_aligned( &self ) -> bool
    {
        self.left_aligned
    }

    pub fn is_right_aligned( &self ) -> bool
    {
        self.right_aligned
    }
    pub fn set_left_aligned( &mut self, aligned: bool )
    {
        self.left_aligned = aligned;
    }

    pub fn set_right_aligned( &mut self, aligned: bool )
    {
        self.right_aligned = aligned;
    }
}

#[derive( PartialEq, Eq, std::hash::Hash, Clone, PartialOrd, Ord, Debug )]
pub struct Tier<'a>
{
    // temporarily public:
    pub slots: Vec<Slot<'a>>,
}

impl<'a> Index<usize> for Tier<'a>
{
    type Output = Slot<'a>;
    fn index( &self, index: usize ) -> &Self::Output
    {
        &self.slots[index]
    }
}

impl<'a> IndexMut<usize> for Tier<'a>
{
    fn index_mut( &mut self, index: usize ) -> &mut Self::Output
    {
        &mut self.slots[index]
    }
}

impl<'a> Tier<'a>
{
    fn concatenate( &self, other: &Self ) -> Self
    {
        let mut slots = Vec::clone( &self.slots );
        Vec::append(
            &mut slots,
            &mut Vec::clone( &other.slots ) );
        Tier { slots }
    }
}

#[derive( PartialEq, Eq, std::hash::Hash, Clone, PartialOrd, Ord )]
pub struct Table<'a>
{
    // temporarily public:
    pub tiers: Vec<Tier<'a>>,
}

impl<'a> Table<'a>
{
    pub fn same_column( &self, col1: usize, col2: usize ) -> bool
    {
        for i in 0..Vec::len( &self.tiers )
        {
            if &self.tiers[i].slots[col1] != &self.tiers[i].slots[col2]
            {
                return false;
            }
        }
        true
    }

    fn eliminate_column( &mut self, slot_index: usize )
    {
        for i in 0..Vec::len( &self.tiers )
        {
            self.tiers[i].slots.remove( slot_index );
        }
    }

    fn merge_cell_with_next(
        &self, tier_index: usize, slot_index: usize ) -> Self
    {
        let mut copy = self.clone();
        copy.tiers[tier_index][slot_index].set_right_aligned( false );
        copy.tiers[tier_index][slot_index+1].set_left_aligned( false );
        let mut found = false;
        // eliminate column if no alignment present:
        for i in 0..Vec::len( &copy.tiers )
        {
            if Slot::is_right_aligned( &copy.tiers[i][slot_index] )
            {
                found = true;
                break;
            }
        }
        if ! found
        {
            Self::eliminate_column( &mut copy, slot_index );
        }
        copy
    }

    fn null_element(
        &self, tier_index: usize, slot_index: usize ) -> Self
    {
        let mut copy = self.clone();
        copy.tiers[tier_index][slot_index].set_gap();
        copy
    }

    // Abstraction based on a single slot in a single tier
    fn abstract_element(
        &self, tier_index: usize, slot_index: usize )
        -> Option<Self>
    {
        // If the slot is a gap, then the only way to
        // abstract it further is to merge it with an
        // eventual gap on its right (or left, but that is
        // the same as merging its left neighbour with it).
        // If merging the two gaps amounts to the deletion
        // of a tab stop, then remove that stop in all tiers.
        let tier = &self.tiers[tier_index];
        // let slot = &tier.slots[slot_index];
        let slot = &tier[slot_index];
        if Slot::is_gap( &slot )
        {
            // is there a slot to its right?
            if Slot::is_right_aligned( &slot ) &&
                ( slot_index < Vec::len( &tier.slots ) - 1 ) &&
                Slot::is_gap( &tier[slot_index + 1] )
            {
                Some(
                    Self::merge_cell_with_next(
                        &self, tier_index, slot_index ) )
            }
            else { None }
        }
        else                    // not a gap
        {
            // All you have to do is converting it into a gap.
            Some(
                Self::null_element(
                    &self, tier_index, slot_index ) )
        }
    }

    fn immediate_abstractions( &self ) -> Set<Self>
    {
        let mut result: Set<Self> = Set::new();
        for i in 0..Vec::len( &self.tiers )
        {
            let tier = &self.tiers[i];
            for j in 0..Vec::len( &tier.slots )
            {
                match Self::abstract_element( &self, i, j )
                {
                    Some( new_tab ) =>
                    {
                        Set::insert( &mut result, new_tab );
                    }
                    _ => ()
                }
            }
        }
        result
    }
}

impl<'a> tabular::Tabular for Table<'a>
{
    type L = &'a str;
    fn new() -> Self
    {
        Table { tiers: Vec::new() }
    }
    fn from_str( input: &'static str ) -> Result<Self, String>
    {
        match table_grammar::table_grammar::table( input )
        {
            Ok( table ) => Ok( table ),
            Err( e ) =>
            {
                println!( "Parse error: {}", &e );
                Err( e.to_string() )
            }
        }
    }
    // presupposes the same types of `Table` instance,
    // same tiers, same elements:
    fn concatenate( &self, other: &Self ) -> Self
    {
        let mut result: Vec<Tier<'a>> = Vec::new();
        for i in 0..Vec::len( &self.tiers )
        {
            Vec::push(
                &mut result,
                Tier::concatenate( &self.tiers[i], &other.tiers[i] ) );
        }
        Table { tiers: result }
    }
    // temporarily public:
    fn abstractions( &self ) -> Set<Self>
    {
        let mut result: Set<Self> = Set::new();
        Set::insert( &mut result, Self::clone( &self ) );
        let mut todo = vec![Self::clone( &self )];
        let mut index = 0;
        loop
        {
            if index == Vec::len( &todo ) { break; }
            for abstr in
                IntoIterator::into_iter(
                    Self::immediate_abstractions( &todo[index] ) )
            {
                match <[_]>::binary_search( &todo, &abstr )
                {
                    Err( _ ) =>
                    {
                        Vec::push(
                            &mut todo,
                            Self::clone( &abstr ) );
                        Set::insert( &mut result, abstr );
                    }
                    _ => ()
                }
            }
            index += 1;
        }
        result
    }
}

impl<'a> fmt::Display for Table<'a>
{
    fn fmt( &self, s: &mut fmt::Formatter ) -> fmt::Result
    {
        write!( s, "
<style type=\"text/css\">
  table, th, td 
  {{
    border-left: 1px solid black;
    border-right: 1px solid black;
    border-collapse: collapse;
    text-align: center;
    padding: 1ex;
  }}
</style>
<table>" ).ok();
        for tier in <[_]>::iter( &self.tiers )
        {
            write!( s, "
  <tr>" ).ok();
            let mut colspan = 1;
            for slot in <[_]>::iter( &tier.slots )
            {
                if Slot::is_right_aligned( &slot )
                {
                    write!( s, "
    <td colspan={}>{}</td>", &colspan, &slot.element ).ok();
                    colspan = 1;
                }
                else
                {
                    colspan += 1;
                }
            }
            write!( s, "
  </tr>" ).ok();
        }
        write!( s, "
</table>\n" )
    }
}
