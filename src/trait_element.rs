use molecule::*;
use trait_properties::*;
use types::*;

use std::hash::Hash;


pub trait Element: Properties + Hash {
    fn get_charge(&self) -> Option<IonCharge>;
    fn get_molecule(&self) -> Option<&Molecule>;
}
