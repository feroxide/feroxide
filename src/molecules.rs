use atoms::*;
use molecule::*;

pub const WATER: &'static Molecule<'static> = &Molecule {
    compounds: &[
        MoleculeCompound { atom: HYDROGEN, amount: 2 },
        MoleculeCompound { atom: OXYGEN, amount: 1 }
    ]
};

pub const TABLE_SALT: &'static Molecule<'static> = &Molecule {
    compounds: &[
        MoleculeCompound { atom: SODIUM, amount: 1 },
        MoleculeCompound { atom: CHLORINE, amount: 1 }
    ]
};

pub const SUGAR: &'static Molecule<'static> = &Molecule {
    compounds: &[
        MoleculeCompound { atom: CARBON, amount: 12 },
        MoleculeCompound { atom: HYDROGEN, amount: 22 },
        MoleculeCompound { atom: OXYGEN, amount: 11 }
    ]
};
