#![allow(non_snake_case)]

use data_atoms::*;
use molecule::*;

/// Due to the disallowance of Rust of vec! in `const`'s, we made these things functions(?)

pub fn WATER() -> Molecule {
    Molecule {
        compounds: vec! {
            MoleculeCompound { atom: HYDROGEN, amount: 2 },
            MoleculeCompound { atom: OXYGEN, amount: 1 }
        }
    }
}

pub fn CO2() -> Molecule {
    Molecule {
        compounds: vec! {
            MoleculeCompound { atom: CARBON, amount: 1 },
            MoleculeCompound { atom: OXYGEN, amount: 2 }
        }
    }
}


pub fn AMMONIA() -> Molecule {
    Molecule {
        compounds: vec! {
            MoleculeCompound { atom: NITROGEN, amount: 1 },
            MoleculeCompound { atom: HYDROGEN, amount: 3 }
        }
    }
}


pub fn SUGAR() -> Molecule {
    Molecule {
        compounds: vec! {
            MoleculeCompound { atom: CARBON, amount: 12 },
            MoleculeCompound { atom: HYDROGEN, amount: 22 },
            MoleculeCompound { atom: OXYGEN, amount: 11 }
        }
    }
}
