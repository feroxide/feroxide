use molecule::*;
use trait_element::*;
use trait_properties::*;
use types::*;


#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Electron { }


pub const ELECTRON: Electron = Electron {};


impl Element for Electron {
    fn get_molecule(&self) -> Option<&Molecule> {
        None
    }

    fn get_charge(&self) -> Option<IonCharge> {
        Some(-1)
    }
}


impl Properties for Electron {
    fn symbol(&self) -> String {
        "e⁻".to_owned()
    }

    fn name(&self) -> String {
        "electron".to_owned()
    }

    fn mass(&self) -> AtomMass {
        0.0
    }
}
