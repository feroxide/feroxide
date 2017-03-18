use atom::*;
use ion::*;
use molecule::*;


#[allow(non_snake_case)]
pub fn ELECTRON() -> Ion {
    Ion {
        molecule: Molecule {
            compounds: vec! {
                MoleculeCompound {
                    atom: Atom {
                        name: "electron",
                        symbol: "e",
                        number: 0,
                        group: 0,
                        mass: 0.0,
                        diatomic: false
                    },
                    amount: 1
                }
            }
        },

        charge: Some(-1)
    }
}


/*
use molecule::*;
use trait_element::*;
use trait_properties::*;
use types::*;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Electron { }


#[allow(non_snake_case)]
pub fn ELECTRON() -> Electron { Electron {} }


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
        // Approximately
        0.0
    }
}

*/
