use ion::*;
use molecule::*;
use atom::*;

use data_atoms::*;
// use data_molecules::*;

pub const ELECTRON: &'static Ion<'static> = &Ion {
    molecule: &Molecule { compounds: &[
        MoleculeCompound {
            atom: &Atom {
                number: 0,
                mass: 0.0,  // approx
                group: 0, // No group
                symbol: "e",
                name: "electron",
                is_diatomic: false
            },
            amount: 1
        }
    ]},
    data: None
};

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
