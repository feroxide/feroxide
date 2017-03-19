use atom::Atom;
use container::{ Container, ContainerCompound };
use ion::Ion;
use molecule::Molecule;
use reaction::{ ElemReaction, ReactionSide, ReactionCompound };
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
        impl<E: Element> Display for $x<E> {
            fn fmt(&self, formatter: &mut Formatter) -> Result {
                write!(formatter, "{}", self.stringify())
            }
        }
    )
}


macro_rules! fmt_lifetime_type {
    ($x:tt) => (
        impl<'lifetime, E: Element> Display for $x<'lifetime, E> {
            fn fmt(&self, formatter: &mut Formatter) -> Result {
                write!(formatter, "{}", self.stringify())
            }
        }
    )
}


fmt!(Atom);
fmt!(Ion);
fmt!(Molecule);
fmt_type!(Container);
fmt_type!(ContainerCompound);
fmt_type!(ElemReaction);
fmt_type!(ReactionCompound);
fmt_type!(ReactionSide);
