use data_atoms::*;
use molecule::{Molecule, MoleculeCompound};


lazy_static! {
    pub static ref WATER: Molecule = Molecule {
        compounds: vec! {
            MoleculeCompound { atom: HYDROGEN, amount: 2 },
            MoleculeCompound { atom: OXYGEN, amount: 1 }
        }
    };


    pub static ref CO2: Molecule = Molecule {
        compounds: vec! {
            MoleculeCompound { atom: CARBON, amount: 1 },
            MoleculeCompound { atom: OXYGEN, amount: 2 }
        }
    };


    pub static ref AMMONIA: Molecule = Molecule {
        compounds: vec! {
            MoleculeCompound { atom: NITROGEN, amount: 1 },
            MoleculeCompound { atom: HYDROGEN, amount: 3 }
        }
    };


    pub static ref SUGAR: Molecule = Molecule {
        compounds: vec! {
            MoleculeCompound { atom: CARBON, amount: 12 },
            MoleculeCompound { atom: HYDROGEN, amount: 22 },
            MoleculeCompound { atom: OXYGEN, amount: 11 }
        }
    };
}
