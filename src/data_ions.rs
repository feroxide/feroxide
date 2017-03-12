use ion::*;
use molecule::*;

use data_atoms::*;
// use data_molecules::*;

pub const HYDROXIDE: &'static Ion<'static> = &Ion {
    molecule: &Molecule { compounds: &[
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


pub const AMMONIUM: &'static Ion<'static> = &Ion {
    molecule: &Molecule { compounds:&[
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

pub const SULPHATE: &'static Ion<'static> = &Ion {
    molecule: &Molecule { compounds: &[
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
