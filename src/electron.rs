use atom::Atom;
use ion::Ion;
use molecule::*;


lazy_static! {

    // NOTE: Since Ion::from_string depends on ELECTRON being available,
    // NOTE: the following will result in an infinite loop:
    // pub static ref ELECTRON: Ion = Ion::from_string("e".to_owned()).unwrap();

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
