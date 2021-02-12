use std::collections::HashSet as Set;
use std::fmt;
use crate::label::Label;
use crate::setoid::Setoid;

// One way of representing setoids is to store
// differences, which are reduced as much as possible.
#[derive( Eq, Clone )]
pub enum Labelset<T: Label>
{
    All,
    None,
    Subset( Set<T> ),           // a finite subset
    Compl( Set<T> ),           // a complement set
}

impl<T: Label> PartialEq for Labelset<T>
{
    fn eq( &self, other: &Labelset<T> ) -> bool
    {
        match self
        {
            Labelset::All => *other == Labelset::All,
            Labelset::None => *other == Labelset::None,
            Labelset::Subset( s ) =>
            {
                match other
                {
                    Labelset::Subset( s2 ) => s == s2,
                    _ => false
                }
            }
            Labelset::Compl( s ) =>
            {
                match other
                {
                    Labelset::Compl( s2 ) => s == s2,
                    _ => false
                }
            }
        }
    }
}

fn display_set<T: Label>( s: &mut fmt::Formatter, set: &Set<T> ) -> fmt::Result
{
    write!( s, "{{" ).ok();
    for el in set
    {
        write!( s, " {}", el ).ok();
    }
    write!( s, " }}" )
}

impl<T: Label> Setoid for Labelset<T>
{
    type El = T;
    fn empty() -> Self
    {
        Labelset::None
    }
    fn is_empty( &self ) -> bool
    {
        *self == Labelset::None
    }
    fn intersection( &self, other: &Self ) -> Self
    {
        match self
        {
            Labelset::All => other.clone(),
            Labelset::None => Labelset::None,
            Labelset::Subset( subs ) =>
            {
                match other
                {
                    Labelset::All =>
                        Labelset::Subset( subs.clone() ),
                    Labelset::None => Labelset::None,
                    Labelset::Subset( other_subs ) =>
                        Labelset::Subset(
                            subs.intersection( &other_subs ).
                                cloned().collect() ),
                    Labelset::Compl( other_subs ) =>
                    {
                        let diff: Set<T> =
                            subs.difference( &other_subs ).
                            cloned().collect();
                        if diff.is_empty()
                        {
                            Labelset::None
                        }
                        else
                        {
                            Labelset::Subset( diff )
                        }
                    }
                }
            }
            Labelset::Compl( subs ) =>
            {
                match other
                {
                    Labelset::All =>
                        Labelset::Compl( subs.clone() ),
                    Labelset::None => Labelset::None,
                    Labelset::Subset( other_subs ) =>
                    {
                        let diff: Set<T> =
                            other_subs.difference( &subs ).
                            cloned().collect();
                        if diff.is_empty()
                        {
                            Labelset::None
                        }
                        else
                        {
                            Labelset::Subset( diff )
                        }
                    }
                    Labelset::Compl( other_subs ) =>
                        Labelset::Compl(
                            subs.union( &other_subs ).
                                cloned().collect() )
                }
            }
        }
    }
    fn union( &self, other: &Self ) -> Self
    {
        match self
        {
            Labelset::All => Labelset::All,
            Labelset::None => other.clone(),
            Labelset::Subset( subs ) =>
            {
                match other
                {
                    Labelset::All => Labelset::All,
                    Labelset::None => self.clone(),
                    Labelset::Subset( s2 ) =>
                        Labelset::Subset(
                            subs.union( &s2 ).cloned().collect() ),
                    Labelset::Compl( s2 ) =>
                        Labelset::Compl( subs.clone() ).
                        intersection(
                            &Labelset::Subset( s2.clone() ) ).
                        complement()
                }
            }
            Labelset::Compl( subs ) =>
            {
                match other
                {
                    Labelset::All => Labelset::All,
                    Labelset::None =>
                        Labelset::Compl( subs.clone() ),
                    Labelset::Subset( s2 ) =>
                        Labelset::Compl( s2.clone() ).
                        intersection(
                            &Labelset::Subset( subs.clone() ) ).
                        complement(),
                    Labelset::Compl( s2 ) =>
                        Labelset::Subset( subs.clone() ).
                        intersection(
                            &Labelset::Subset( s2.clone() ) ).
                        complement()
                }
            }
        }
    }
    fn difference( &self, other: &Self ) -> Self
    {
        self.intersection( &other.complement() )
    }
    fn complement( &self ) -> Self
    {
        match self
        {
            Labelset::All => Labelset::None,
            Labelset::None => Labelset::All,
            Labelset::Subset( subs ) =>
                Labelset::Compl( subs.clone() ),
            Labelset::Compl( subs ) =>
                Labelset::Subset( subs.clone() )
        }
    }
    fn member( &self, el: &T ) -> bool
    {
        match self
        {
            Labelset::All => true,
            Labelset::None => false,
            Labelset::Subset( subs ) => subs.contains( el ),
            Labelset::Compl( subs ) => ! subs.contains( el )
        }
    }
}

impl<T: Label> fmt::Display for Labelset<T>
{
    fn fmt( &self, s: &mut fmt::Formatter ) -> fmt::Result
    {
        match self
        {
            Labelset::All => write!( s, "*" ),
            Labelset::None => write!( s, "0" ),
            Labelset::Subset( subs ) => display_set( s, &subs ),
            Labelset::Compl( subs ) =>
            {
                write!( s, "~" ).ok();
                display_set( s, &subs )
            }
        }
    }
}
