use atom::Atom;
use container::Container;
use electron::Electron;
use ion::Ion;
use molecule::Molecule;
use reaction::{ Reaction, ReactionSide, ReactionCompound };
use trait_element::Element;
use trait_properties::Properties;

use std::fmt::{ Display, Formatter, Result };


macro_rules! fmt {
    ($x:tt) => (
        impl Display for $x {
            fn fmt(&self, formatter: &mut Formatter) -> Result {
                write!(formatter, "{}", self.stringify())
            }
        }
    )
}

macro_rules! fmt_lifetime {
    ($x:tt) => (
        impl<'lifetime> Display for $x<'lifetime> {
            fn fmt(&self, formatter: &mut Formatter) -> Result {
                write!(formatter, "{}", self.stringify())
            }
        }
    )
}

macro_rules! fmt_type {
    ($x:tt) => (
        impl<T> Display for $x<T> where T: Element {
            fn fmt(&self, formatter: &mut Formatter) -> Result {
                write!(formatter, "{}", self.stringify())
            }
        }
    )
}

macro_rules! fmt_lifetime_type {
    ($x:tt) => (
        impl<'lifetime, T> Display for $x<'lifetime, T> where T: Element {
            fn fmt(&self, formatter: &mut Formatter) -> Result {
                write!(formatter, "{}", self.stringify())
            }
        }
    )
}


fmt!(Atom);
fmt!(Electron);
fmt_lifetime!(Ion);
fmt_lifetime!(Molecule);
fmt_type!(Container);
fmt_type!(ReactionCompound);
fmt_lifetime_type!(Reaction);
fmt_lifetime_type!(ReactionSide);
