use std::fmt::Display;
use std::fmt::Debug;
use std::hash::Hash;

pub trait Label :
Ord + Eq + PartialEq + Clone + Hash + Display + Debug
{
}
