use std::fmt;
use std::collections::HashMap as Map;
use std::collections::HashSet as Set;
// Here we don't use setoids, labels are simple unanalysed entities,
// the representation consists of synchronized tiers.

pub trait Label: Eq + std::hash::Hash + Copy + fmt::Display
{
    fn wild() -> Self;
    fn is_wild( &self ) -> bool;
}

impl Label for u32
{
    fn wild() -> Self { 0 }
    fn is_wild( &self ) -> bool { *self == 0 }
}

// Mapping from indices to indices as a result of
// operations on machines.
pub trait IndexMap
{
    fn new() -> Self;
}

impl IndexMap for Map<u32, Set<u32>>
{
    fn new() -> Self { Map::new() }
}

pub trait Machine<L: Label, M: IndexMap> : fmt::Display + Sized
{
    fn new() -> Self;
    // Also returns index maps from the two arguments
    // to the value:
    fn intersection(
        machine1: &Self, machine2: &Self ) ->
        (Self, M, M);
}



// This type of FSAs does not contain loops, except
// self-loops that accept any sequence except with
// labels that are explicitly present in transitions
// (and they can be used only in automata operations,
// not in actual accepting mode).
pub struct FSA<L: Label>
{
    // temporarily public:
    pub starting: u32,
    pub accepting: Set<u32>,
    pub looping: Set<u32>,
    transitions: Map<u32, Map<L, u32>>,
    largest_state: u32,
}

// A `Score` instance is like an autosegmental representation.
// It is a partial synchronization of several `FSA`s (“tiers”)
// using “points”.  The following must hold:
// 1. A state in a tier is associated with at most one point.
// 2. Each point is associated with at least two tiers, where
// 3. A point is associated with a tier if the tier contains
//    a state associated with it.
// 4. Points associated with a tier are linearly ordered.
// 5. A point is associated with at most one state of a tier.

// In `FSA` operations on `Score`s, we will assume that the
// number of tiers in them is identical.
pub struct Score<L: Label>
{
    tiers: Vec<FSA<L>>,
    tier_to_state_to_point: Map<u32, Map<u32, u32>>,
    point_to_tiers: Map<u32, Set<u32>>,
    point_to_followers: Map<u32, Set<u32>>,
    next_point: u32,
}

impl<L: Label> FSA<L>
{
    // temporarily made public:
    pub fn add_edge( &mut self, from_node: u32, label: &L, to_node: u32 )
    {
        let trs =
            self.transitions.get_mut( &from_node ).unwrap();
        let _ = trs.entry( *label ).or_insert( to_node );
    }
    // temporarily made public:
    pub fn add_node( &mut self ) -> u32
    {
        let state = self.largest_state + 1;
        self.largest_state = state;
        self.transitions.insert( state, Map::new() );
        state
    }
    fn remove_state( &mut self, state: &u32 )
    {
        self.looping.remove( state );
        self.transitions.remove( state );
        let mut to_remove: Set<(u32, L)> = Set::new();
        for from_state in self.transitions.keys()
        {
            for (label, endpoint) in self.transitions[from_state].iter()
            {
                if endpoint == state
                {
                    to_remove.insert( (*from_state, *label) );
                }
            }
        }
        for (from_state, label) in to_remove.iter()
        {
            self.transitions.entry( *from_state ).
                and_modify( |m| { m.remove( label ); } );
        }
    }
    
    fn leave_out_useless_states( &mut self )
    {
        // 1. non-accepting states without transitions are useless.
        // 2. transitions leading to useless states are useless.
        // 3. repeat until only useful states remain.
        loop
        {
            let mut useless: Set<u32> = Set::new();
            for state in self.transitions.keys()
            {
                if useless.contains( state ) { continue; }
                if ( ! self.accepting.contains( state ) ) &&
                    self.transitions[state].is_empty()
                {
                    useless.insert( *state );
                }
            }
            if useless.is_empty() { break; }
            else
            {
                for useless_state in useless.iter()
                {
                    self.remove_state( useless_state );
                }
            }
        }
    }

    fn intersection_loop(
        machine1: &FSA<L>, machine2: &FSA<L>,
        result: &mut FSA<L>,
        map1: &mut Map<u32, Set<u32>>,
        map2: &mut Map<u32, Set<u32>>,
        todo: &mut Vec<(u32, u32, u32)> )
    {
        let mut index = 0;
        loop
        {
            if index == todo.len() - 1 { break; }
            let next_state1 = todo[index].0;
            let next_state2 = todo[index].1;
            let result_state = todo[index].2;
            let transitions1 = &machine1.transitions[&next_state1];
            let transitions2 = &machine2.transitions[&next_state2];
            for (label1, endpoint1) in transitions1.iter()
            {
                if transitions2.contains_key( label1 )
                {
                    let endpoint2 = &transitions2[label1];
                    FSA::intersection_both_have_edge(
                        machine1, machine2, result,
                        map1, map2, todo,
                        label1, endpoint1, endpoint2,
                        &result_state )
                }
                else
                {
                    FSA::intersection_only_first_has_edge(
                        machine1, machine2, result,
                        map1, map2, todo,
                        label1, endpoint1, next_state2,
                        result_state );
                }
            }
            for (label2, endpoint2) in transitions2.iter()
            {
                if transitions1.contains_key( label2 ) { continue; }
                // only in machine2:
                FSA::intersection_only_second_has_edge(
                    machine1, machine2, result,
                    map1, map2, todo,
                    label2, next_state1, endpoint2,
                    result_state );
            }
            index += 1;
        }
        result.leave_out_useless_states();
    }
    fn intersection_both_have_edge(
        machine1: &FSA<L>, machine2: &FSA<L>,
        result: &mut FSA<L>,
        map1: &mut Map<u32, Set<u32>>,
        map2: &mut Map<u32, Set<u32>>,
        todo: &mut Vec<(u32, u32, u32)>,
        label1: &L, endpoint1: &u32, endpoint2: &u32,
        result_state: &u32 )
    {
        if result.transitions[&result_state].
            contains_key( label1 )
        {
            let next_node =
                result.transitions[&result_state][label1];
            FSA::intersection_both_and_result_have_edge(
                machine1, machine2, result,
                map1, map2, todo,
                endpoint1, endpoint2, next_node );
        }
        else
        {
            let next_node = result.add_node();
            result.add_edge(
                *result_state, label1, next_node );
            FSA::intersection_both_but_not_result_have_edge(
                machine1, machine2, result,
                map1, map2, todo,
                endpoint1, endpoint2, next_node );
        }
    }
                        
    fn intersection_only_first_has_edge(
        machine1: &FSA<L>, machine2: &FSA<L>,
        result: &mut FSA<L>,
        map1: &mut Map<u32, Set<u32>>,
        map2: &mut Map<u32, Set<u32>>,
        todo: &mut Vec<(u32, u32, u32)>,
        label1: &L, endpoint1: &u32, next_state2: u32,
        result_state: u32 )
    {
        // only in machine1; if next_state2 is looping,
        // still feasible:
        if machine2.looping.contains( &next_state2 )
        {
            let next_node =
                if result.transitions[&result_state].
                contains_key( label1 )
            {
                result.transitions[&result_state][label1]
            }
            else
            {
                let node = result.add_node();
                result.add_edge(
                    result_state, label1, node );
                node
            };
            if machine1.accepting.contains( endpoint1 ) &&
                machine2.accepting.contains( &next_state2 )
            {
                result.accepting.insert( next_node );
            }
            if machine1.looping.contains( endpoint1 )
            {
                result.looping.insert( next_node );
            }
            todo.push( (*endpoint1, next_state2, next_node) );
            let ep1map =
                map1.entry( *endpoint1 ).
                or_insert( Set::new() );
            ep1map.insert( next_node );
            let ns2map =
                map2.entry( next_state2 ).
                or_insert( Set::new() );
            ns2map.insert( next_node );
        }
    }

    fn intersection_only_second_has_edge(
        machine1: &FSA<L>, machine2: &FSA<L>,
        result: &mut FSA<L>,
        map1: &mut Map<u32, Set<u32>>,
        map2: &mut Map<u32, Set<u32>>,
        todo: &mut Vec<(u32, u32, u32)>,
        label2: &L, next_state1: u32, endpoint2: &u32,
        result_state: u32 )
    {
        if machine1.looping.contains( &next_state1 )
        {
            let node =
                if result.transitions[&result_state].
                contains_key( label2 )
            {
                result.transitions[&result_state][label2]
            }
            else
            {
                let next_node = result.add_node();
                result.add_edge(
                    result_state, label2, next_node );
                next_node
            };
            if machine1.accepting.contains( &next_state1 ) &&
                machine2.accepting.contains( endpoint2 )
            {
                result.accepting.insert( node );
            }
            if machine2.looping.contains( endpoint2 )
            {
                result.looping.insert( node );
            }
            todo.push( (next_state1, *endpoint2, node) );
            let ns1map =
                map1.entry( next_state1 ).
                or_insert( Set::new() );
            ns1map.insert( node );
            let ep2map =
                map2.entry( *endpoint2 ).
                or_insert( Set::new() );
            ep2map.insert( node );
        }
    }

    fn intersection_both_and_result_have_edge(
        machine1: &FSA<L>, machine2: &FSA<L>,
        result: &mut FSA<L>,
        map1: &mut Map<u32, Set<u32>>,
        map2: &mut Map<u32, Set<u32>>,
        todo: &mut Vec<(u32, u32, u32)>,
        endpoint1: &u32, endpoint2: &u32,
        next_node: u32 )
    {
        if ( ! machine1.accepting.
               contains( endpoint1 ) ) ||
            ( ! machine2.accepting.contains( endpoint2 ) )
        {
            result.accepting.remove( &next_node );
        }
        if ( ! machine1.looping.contains( endpoint1 ) ) ||
            ( ! machine2.looping.contains( endpoint2 ) )
        {
            result.looping.remove( &next_node );
        }
        let ep1map =
            map1.entry( *endpoint1 ).
            or_insert( Set::new() );
        ep1map.insert( next_node );
        let ep2map =
            map2.entry( *endpoint2 ).
            or_insert( Set::new() );
        ep2map.insert( next_node );
        let triplet =
            (*endpoint1, *endpoint2, next_node);
        match todo.binary_search( &triplet )
        {
            Err( _ ) =>
            {
                todo.push( triplet );
            }
            _ => ()
        }
    }
            
    fn intersection_both_but_not_result_have_edge(
        machine1: &FSA<L>, machine2: &FSA<L>,
        result: &mut FSA<L>,
        map1: &mut Map<u32, Set<u32>>,
        map2: &mut Map<u32, Set<u32>>,
        todo: &mut Vec<(u32, u32, u32)>,
        endpoint1: &u32, endpoint2: &u32,
        next_node: u32 )
    {
        if machine1.accepting.contains( endpoint1 ) &&
            machine2.accepting.contains( endpoint2 )
        {
            result.accepting.insert( next_node );
        }
        if machine1.looping.contains( endpoint1 ) &&
            machine2.looping.contains( endpoint2 )
        {
            result.looping.insert( next_node );
        }
        let ep1map =
            map1.entry( *endpoint1 ).
            or_insert( Set::new() );
        ep1map.insert( next_node );
        let ep2map =
            map2.entry( *endpoint2 ).
            or_insert( Set::new() );
        ep2map.insert( next_node );
        let triplet =
            (*endpoint1, *endpoint2, next_node);
        match todo.binary_search( &triplet )
        {
            Err( _ ) =>
            {
                todo.push( triplet );
            }
            _ => ()
        }
    }
}

impl<L: Label> Machine<L, Map<u32, Set<u32>>> for FSA<L>
{
    fn new() -> Self
    {
        FSA
        {
            starting: 0,
            accepting: Set::new(),
            looping: Set::new(),
            transitions:
            vec![(0, Map::new())].into_iter().collect(),
            largest_state: 0,
        }
    }
    fn intersection(
        machine1: &FSA<L>, machine2: &FSA<L> ) ->
        (FSA<L>, Map<u32, Set<u32>>, Map<u32, Set<u32>>)
    {
        let mut result = FSA::new();
        let mut map1: Map<u32, Set<u32>> = Map::new();
        let mut map2: Map<u32, Set<u32>> = Map::new();
        map1.insert(
            machine1.starting,
            vec![result.starting].into_iter().collect() );
        map2.insert(
            machine2.starting,
            vec![result.starting].into_iter().collect() );
        if machine1.accepting.contains( &machine1.starting ) &&
            machine2.accepting.contains( &machine2.starting )
        {
            result.accepting.insert( result.starting );
        }
        if machine1.looping.contains( &machine1.starting ) &&
            machine2.looping.contains( &machine2.starting )
        {
            result.looping.insert( result.starting );
        }
        let mut todo =
            vec![(machine1.starting,
                  machine2.starting,
                  result.starting)];
        FSA::intersection_loop(
            &machine1, &machine2, &mut result,
            &mut map1, &mut map2, &mut todo );
        (result, map1, map2)
    }
}

impl<L: Label> fmt::Display for FSA<L>
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
                    for (l, endpoint) in
                        self.transitions[&node].iter()
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
            write!( s, "\n" ).ok();
        }
        Ok( () )
    }
}
