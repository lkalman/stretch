pub trait Lattice<T: Eq + PartialOrd>
{
    fn top( &self ) -> T;
    fn bottom( &self ) -> T;
    // infimum:
    fn meet( &self, el1: &T, el2: &T ) -> T;
    // supremum:
    fn join( &self, el1: &T, el2: &T ) -> T;
}
