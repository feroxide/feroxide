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


    println!("{} is called \"{}\" and has mass {}", water.symbol(), water.name(), water.mass());
    println!("{} is called \"{}\" and has mass {}", hcl.symbol(), hcl.name(), hcl.mass());
}
