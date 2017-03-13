use molecule::*;
use ion::*;
use data_atoms::*;


pub const HYDROXIDE: Ion<'static> = Ion {
    molecule: Molecule { compounds: &[
        MoleculeCompound {
            atom: OXYGEN,
            amount: 1
        },
        MoleculeCompound {
            atom: HYDROGEN,
            amount: 1
        }
    ]},
    data: None
};


pub const AMMONIUM: Ion<'static> = Ion {
    molecule: Molecule { compounds:&[
        MoleculeCompound {
            atom: NITROGEN,
            amount: 1
        },
        MoleculeCompound {
            atom: HYDROGEN,
            amount: 4
        }
    ]},
    data: None
};

pub const SULPHATE: Ion<'static> = Ion {
    molecule: Molecule { compounds: &[
        MoleculeCompound {
            atom: SULFUR,
            amount: 1
        },
        MoleculeCompound {
            atom: OXYGEN,
            amount: 4
        }
    ]},
    data: None
};
