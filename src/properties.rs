use types::*;

use std::fmt::Debug;


pub trait Properties: Debug {
    fn symbol(&self) -> String;
    fn name(&self) -> String;
    fn mass(&self) -> AtomMass;

    fn to_string(&self) -> String { self.symbol() }
}
