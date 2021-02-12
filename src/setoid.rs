use std::collections::HashSet as Set;
use std::fmt;

/// A `Setoid` is almost like a set, but there is no way
/// we can enumerate its elements, so it can be infinite.
pub trait Setoid : fmt::Display
{
    type El: Eq;
    fn empty() -> Self;
    fn is_empty( &self ) -> bool;
    fn intersection( &self, other: &Self ) -> Self;
    fn union( &self, other: &Self ) -> Self;
    fn difference( &self, other: &Self ) -> Self;
    fn complement( &self ) -> Self;
    fn member( &self, el: &Self::El ) -> bool;
}
