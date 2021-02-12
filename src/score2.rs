use std::fmt;
use std::collections::HashMap as Map;
use std::collections::HashSet as Set;
mod label;
mod subsets;
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


struct Score<L: Label>( Set<Vec<Vec<L>>> );

// we know that `vec` is not empty
// the result must consist of vectors of the same size as `vec`
fn subvecs<L: Label>( vec: &Vec<Vec<L>> ) -> Vec<Vec<Vec<L>>>
{
    // in the beginning, `result` consists of a vector
    // of size 0, but in each iteration, we replace it with
    // vectors one bigger than in the previous one
    let mut result: Vec<Vec<Vec<L>>> = vec![vec![]];
    for labelset in vec
    {
        let mut new_result: Vec<Vec<Vec<L>>> = Vec::new();
        for label_subset in subsets::subsets( &labelset )
        {
            if label_subset.len() == 0 { continue; }
            for previous_vec in result.iter()
            {
                let mut beginning = previous_vec.clone();
                beginning.push( label_subset.clone() );
                new_result.push( beginning );
            }
        }
        result = new_result;
    }
    result
}

impl<L: Label> Score<L>
{
    fn substrings( &self ) -> Set<Vec<Vec<L>>>
    {
        let mut result: Set<Vec<Vec<L>>> = Set::new();
        let Score( vs ) = self;
        for v in vs.iter()
        {
            for vec in subsets::subsets( v )
            {
                if vec.len() == 0 { continue; }
                // we have to take every vector
                // of the same length as `vec`,
                // and each element a non-empty
                // subset of the corresponding
                // element of `vec`:
                for subvec in subvecs( &vec ).iter()
                {
                    if ! result.contains( subvec )
                    {
                        result.insert( subvec.clone() );
                    }
                }
            }
        }
        result
    }
}

impl Label for u32 { }

fn main()
{
    let s: Vec<Vec<Vec<u32>>> = vec![vec![vec![0, 1], vec![2, 3]]];
    let score = Score::<u32>( s.iter().cloned().collect() );
    println!( "{:?} => {:?}", &s, &score.substrings() );
}
