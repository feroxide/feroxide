#![allow(non_snake_case)]

use data_atoms::*;
use ion::*;
use molecule::*;

/// Due to the disallowance of Rust of vec! in `const`'s, we made these things functions(?)

pub fn HYDROXIDE() -> Ion {
    Ion {
        molecule: Molecule { compounds: vec! {
            MoleculeCompound {
                atom: OXYGEN,
                amount: 1
            },

            MoleculeCompound {
                atom: HYDROGEN,
                amount: 1
            }
        }},

        charge: None
    }
}


pub fn AMMONIUM() -> Ion {
    Ion {
        molecule: Molecule { compounds: vec! {
            MoleculeCompound {
                atom: NITROGEN,
                amount: 1
            },

            MoleculeCompound {
                atom: HYDROGEN,
                amount: 4
            }
        }},

        charge: None
    }
}


pub fn SULPHATE() -> Ion {
    Ion {
        molecule: Molecule { compounds: vec! {
            MoleculeCompound {
                atom: SULFUR,
                amount: 1
            },

            MoleculeCompound {
                atom: OXYGEN,
                amount: 4
            }
        }},

        charge: None
    }
}
