use atom::{ Atom };
use ion::{ Ion };
use molecule::{ Molecule, MoleculeCompound };


lazy_static! {
    pub static ref ELECTRON: Ion = Ion {
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
    };
}
