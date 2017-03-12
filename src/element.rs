use molecule::*;
use properties::*;

use types::*;

pub trait Element: Properties + Eq + PartialEq {
    fn get_charge(&self) -> Option<IonCharge> { Some(0) }
    fn get_molecule(&self) -> Option<&Molecule>;
}
