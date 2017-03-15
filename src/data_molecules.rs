use data_atoms::*;
use molecule::*;


pub const WATER: Molecule<'static> = Molecule {
    compounds: &[
        MoleculeCompound { atom: HYDROGEN, amount: 2 },
        MoleculeCompound { atom: OXYGEN, amount: 1 }
    ]
};

pub const CO2: Molecule<'static> = Molecule {
    compounds: &[
        MoleculeCompound { atom: CARBON, amount: 1 },
        MoleculeCompound { atom: OXYGEN, amount: 2 }
    ]
};

pub const AMMONIA: Molecule<'static> = Molecule {
    compounds: &[
        MoleculeCompound { atom: NITROGEN, amount: 1 },
        MoleculeCompound { atom: HYDROGEN, amount: 3 }
    ]
};

pub const SUGAR: Molecule<'static> = Molecule {
    compounds: &[
        MoleculeCompound { atom: CARBON, amount: 12 },
        MoleculeCompound { atom: HYDROGEN, amount: 22 },
        MoleculeCompound { atom: OXYGEN, amount: 11 }
    ]
};

/*
// Disabled because it should be classified as a Salt

pub const TABLE_SALT: &'static Molecule<'static> = &Molecule {
    compounds: &[
        MoleculeCompound { atom: SODIUM, amount: 1 },
        MoleculeCompound { atom: CHLORINE, amount: 1 }
    ]
};
*/
