use std::fmt;
use std::collections::HashMap as Map;
use std::collections::HashSet as Set;

use crate::label::Label;

// A `Score` instance is a set of vectors of sets of autosegments
// (an autosegment can be any `Label` instance, assuming that
// the sets of autosegments that may appear on different tiers
// are pairwise disjoint).  These vectors represent the possible
// realizations of an autosegmental representation.  For example,
// The representation
// H    L
//  \  /
//   V
// can be realized as <HV, LV>.  That is, we assume that an association
// line between X and Y prescribes the alignment of both the beginning
// and end of X and Y, unless one and/or the other has another association
// line (between their tiers).  For example, if we have
//   H    L
//  / \  /
// V_1 V_2
// Here Beg(V_1) = Beg(H) and End(V_2) = End(L),
// but the end of H is not aligned with the end of V_1,
// and the beginning of L is not aligned with the
// beginning of V_2.  So the only possible realization is
// <HV_1, HV2, LV2>.

// Multiple realizations are possible when no alignment is specified
// between autosegments.  For example, if we have
// H   L
//     |
// V_1 V_2
// This can be realized as <H, HV_1, LV_2>,  <V_1, HV_1, LV_2> or
// <HV_1, LV_2>.  (Are <H, V1, LV2> and <V1, H, LV2> possible
// realizations?  Are <H, HV_1, H, LV2> and <V_1, H, HV_1, LV2>???
// Do we need such degenerated representations at all?

// When several tiers are allowed, it is more frequent that not
// all alignments are determined.


pub struct Score<S: Setoid>
{
    starting: u32,
    accepting: Set<u32>,
    looping: Set<u32>,
    transitions: Map<u32, Map<S, Set<u32>>>,
    next_state: u32,
}

impl<S: Setoid> Score<S>
{
    fn intersection_loop(
        machine1: &Self, machine2: &Self,
        result: &mut Self,
        map1: &mut Map<u32, Set<u32>>,
        map2: &mut Map<u32, Set<u32>>,
        todo: &mut Vec<(u32, u32, u32)> )
    {
        let mut index = 0;
        loop
        {
            if index == todo.len() { break; }
            let state1 = todo[index].0; 
            let state2 = todo[index].1;
            let state = todo[index].2;
            let transs1 = &machine1.transitions[&state1]; 
            let transs2 = &machine2.transitions[&state2];
            for (label1, endpoints) in transs1.iter()
            {
                // missing
            }
            for (label2, endpoints) in transs2.iter()
            {
                /*
                if transs1.contains_key( label2 ) { continue; }
                
                // only in machine2:
                */
                // missing
            }
            index += 1;
        }
        // result.leave_out_useless_states();
        // result.collapse_equivalent_states();
    }
}

impl<S: Setoid> Representation for Score<S>
{
    fn new() -> Self
    {
        Score
        {
            starting: 0,
            accepting: Set::new(),
            looping: Set::new(),
            transitions: Map::new(),
            next_state: 1,
        }
    }
    fn intersection( repr1: &Self, repr2: &Self ) -> Self
    {
        let mut result = Self::new();
        let mut map1: Map<u32, Set<u32>> = Map::new();
        let mut map2: Map<u32, Set<u32>> = Map::new();
        map1.insert(
            repr1.starting,
            vec![result.starting].into_iter().collect() );
        map1.insert(
            repr2.starting,
            vec![result.starting].into_iter().collect() );
        if repr1.accepting.contains( &repr1.starting ) &&
            repr2.accepting.contains( &repr2.starting )
        {
            result.accepting.insert( result.starting );
        }
        if repr1.looping.contains( &repr1.starting ) &&
            repr2.looping.contains( &repr2.starting )
        {
            result.looping.insert( result.starting );
        }
        let mut todo =
            vec![(repr1.starting, repr2.starting, result.starting)];
        Score::intersection_loop(
            repr1, repr2, &mut result,
            &mut map1, &mut map2, &mut todo );
        result
    }
}

impl<S: Setoid> fmt::Display for Score<S>
{
    fn fmt( &self, s: &mut fmt::Formatter ) -> fmt::Result
    {
        let mut printed: Set<u32> = Set::new();
        let mut to_print: Vec<u32> = vec![self.starting];
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
                    if self.looping.contains( &node )
                    {
                        write!( s, "$" ).ok();
                    }
                    for (l, endpoints) in
                        self.transitions[&node].iter()
                    {
                        for endpoint in endpoints
                        {
                            write!( s, " {} -> {}", l, endpoint ).ok();
                            if ! printed.contains( endpoint )
                            {
                                to_print.insert( 0, *endpoint );
                            }
                        }
                    }
                    printed.insert( node );
                }
            }
            write!( s, "\n" ).ok();
        }
        Ok( () )
    }
}
