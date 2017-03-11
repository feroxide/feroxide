extern crate feroxide;

use feroxide::{ MoleculeCompound, Molecule };
use feroxide::atoms::*;

fn main() {
    println!("Hydrogen: {}", HYDROGEN.data());

    let water = Molecule {
        compounds: &vec! {
            MoleculeCompound { atom: HYDROGEN, amount: 2 },
            MoleculeCompound { atom: OXYGEN, amount: 1 }
        }
    };

    let hcl = Molecule {
        compounds: &vec! {
            MoleculeCompound { atom: HYDROGEN, amount: 1 },
            MoleculeCompound { atom: CHLORINE, amount: 1 }
        }
    };

    let sugar = Molecule {
        compounds: &vec! {
            MoleculeCompound { atom: CARBON, amount: 12 },
            MoleculeCompound { atom: HYDROGEN, amount: 22 },
            MoleculeCompound { atom: OXYGEN, amount: 11 }
        }
    };


    println!("{} is called \"{}\" and has mass {}", water.symbol(), water.name(), water.mass());
    println!("{} is called \"{}\" and has mass {}", hcl.symbol(), hcl.name(), hcl.mass());
    println!("{} is called \"{}\" and has mass {}", sugar.symbol(), sugar.name(), sugar.mass());
}
