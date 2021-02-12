use std::fmt;

pub trait Representation : fmt::Display + Sized
{
    fn new() -> Self;
    fn intersection( repr1: &Self, repr2: &Self ) -> Self;
}
