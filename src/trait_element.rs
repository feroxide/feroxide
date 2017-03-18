use molecule::*;
use trait_properties::*;
use types::*;

use std::hash::Hash;


pub trait Element: Properties + Hash {
    /// Get the charge of the current Element
    fn get_charge(&self) -> Option<IonCharge>;


    /// Get the molecule associated with the current Element
    fn get_molecule(&self) -> Option<&Molecule>;
}
