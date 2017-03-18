#![allow(non_snake_case)]

use data_atoms::*;
use ion::*;
use molecule::*;


lazy_static! {
    pub static ref HYDROXIDE: Ion = Ion {
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
    };

    pub static ref AMMONIUM: Ion = Ion {
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
    };

    pub static ref SULPHATE: Ion = Ion {
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
    };
}
