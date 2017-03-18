use types::*;

use std::fmt::Debug;


pub trait Properties: Debug + Eq + PartialEq + Clone {
    /// Get the symbol of the current item
    fn symbol(&self) -> String;


    /// Get the name of the current item
    fn name(&self) -> String;


    /// Get the mass of the current item
    fn mass(&self) -> AtomMass;


    /// Stringify the current item (default: self.symbol())
    fn stringify(&self) -> String {
        self.symbol()
    }
}
