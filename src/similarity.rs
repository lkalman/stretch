use std::collections::HashSet as Set;
use std::collections::HashMap as Map;
use std::hash::Hash;

use crate::subsets;

/// Similarity of two vectors in terms of
/// (possibly discontinuous) substrings

fn all_substrings_present<T: Eq + Hash + Clone + std::fmt::Debug>(
    vecs: &Set<Vec<T>>,
    vec_to_substrings: &Map<Vec<T>, Set<Vec<T>>> ) ->
    Set<Vec<T>>
{
    let mut result: Set<Vec<T>> = Set::new();
    for vec in vecs.iter()
    {
        let mut found_all = true;
        for substring in vec_to_substrings.get( vec ).unwrap()
        {
            if ! vecs.contains( substring )
            {
                found_all = false;
                break;
            }
        }
        if found_all
        {
            result.insert( vec.clone() );
        }
    }
    result
}    

fn no_larger_candidate<T: Eq + Hash + Clone + std::fmt::Debug>(
    all_substrings_present: &Set<Vec<T>>,
    vec_to_substrings: &Map<Vec<T>, Set<Vec<T>>> ) ->
    Set<Vec<T>>
{
    let mut result: Set<Vec<T>> = Set::new();
    for vec in all_substrings_present.iter()
    {
        let mut found_larger = false;
        for other_vec in all_substrings_present.iter()
        {
            if vec == other_vec { continue; }
            if vec_to_substrings.get( other_vec ).unwrap().
                contains( vec )
            {
                found_larger = true;
                break;
            }
        }
        if ! found_larger
        {
            result.insert( vec.clone() );
        }
    }
    result
}

// missing: cannot we ever relax these severe criteria on
// candidates?  (The current criteria are: all substrings
// of a candidate must be present, and the candidate cannot
// be a substring of another candidate.)
pub fn similarity_candidates<T: Eq + Hash + Clone + std::fmt::Debug>(
    vecs: &Set<Vec<T>> ) ->
    Set<Vec<T>>
{
    let mut vec_to_substrings: Map<Vec<T>, Set<Vec<T>>> = Map::new();
    for vec in vecs.iter()
    {
        vec_to_substrings.insert(
            vec.clone(),
            subsets::subsets( vec ).iter().cloned().collect() );
    }
    let all_substrings_present =
        all_substrings_present( vecs, &vec_to_substrings );
    no_larger_candidate( &all_substrings_present,
                          &vec_to_substrings )
}

pub fn similarity<T: Eq + Hash + Clone + std::fmt::Debug>( vec1: &Vec<T>, vec2: &Vec<T> ) ->
    Set<Vec<T>>
{
    let ss1: Set<Vec<T>> =
        subsets::subsets( vec1 ).iter().cloned().collect();
    let ss2: Set<Vec<T>> =
        subsets::subsets( vec2 ).iter().cloned().collect();
    let inters: Set<Vec<T>> =
        ss1.intersection( &ss2 ).cloned().collect();
    // get maximal consistent subsets:
    // missing: there must be a more efficient way of doing this...
    similarity_candidates( &inters )
}

