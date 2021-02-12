use std::collections::HashSet as Set;
use std::collections::HashMap as Map;
use std::marker::PhantomData;
use std::fmt;

use crate::setoid::Setoid;
use crate::label::Label;

// Automaton, transitions are labelled with labels implementing
// `setoid::Setoid`.

pub trait Automaton<T: Label, L: Setoid<T>>
{
    fn new() -> Self;
}

pub struct Aut<T:Label, L: Setoid<T>>
{
    starting_state: u32,
    accepting: Set<u32>,
    transitions: Map<u32, Map<L, u32>>,
    phantom_label: PhantomData<T>,
}

impl<T: Label, L: Setoid<T>> Automaton<T, L> for Aut<T, L>
{
    fn new() -> Self
    {
        Aut
        {
            starting_state: 0,
            accepting: Set::new(),
            transitions: Map::new(),
            phantom_label: PhantomData,
        }
    }
}

impl<T: Label, L: Setoid<T>> fmt::Display for Aut<T, L>
{
    fn fmt( &self, s: &mut fmt::Formatter ) -> fmt::Result
    {
        let mut printed: Set<u32> = Set::new();
        let mut to_print: Vec<u32> = vec![self.starting_state];
        loop
        {
            match to_print.pop()
            {
                None => { break; }
                Some( node ) =>
                {
                    write!( s, "{}", node ).ok();
                    if self.accepting.contains( &node )
                    {
                        write!( s, "." ).ok();
                    }
                    for (l, endpoint) in self.transitions[&node].iter()
                    {
                        write!( s, " {} -> {}", l, endpoint ).ok();
                        if ! printed.contains( endpoint )
                        {
                            to_print.insert( 0, *endpoint );
                        }
                    }
                    printed.insert( node );
                }
            }
        }
        Ok( () )
    }
}

// For operations on synchronized machines, we assume
// they always contain the same number of tiers, and
// operations proceed tier by tier.
pub struct SynchronizedAuts<T: Label, L: Setoid<T>>
{
    tiers: Vec<Aut<T, L>>,
    points: Vec<u32>,
    machine_to_state_to_point: Map<u32, Map<u32, u32>>,
}

impl<T: Label, L: Setoid<T>> SynchronizedAuts<T, L>
{
    // temporary:
    fn new() -> Self
    {
        SynchronizedAuts
        {
            tiers: Vec::new(),
            points: Vec::new(),
            machine_to_state_to_point: Map::new()
        }
    }
    fn intersection( &self, other: &SynchronizedAuts<T, L> ) ->
        Self
    {
        // missing:
        SynchronizedAuts::new()
    }
}
