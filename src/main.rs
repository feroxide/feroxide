extern crate feroxide;

use feroxide::{ Molecule, MoleculeCompound, Reaction, ReactionSide, ReactionCompound };

use feroxide::atoms::*;
use feroxide::molecules::*;

macro_rules! molecule_from_atom {
    ($atom:expr) => (
        Molecule { compounds: &[ MoleculeCompound::from_atom($atom) ] }
    )
}

fn main() {
    println!("Hydrogen: {}", HYDROGEN.data());
    println!("{} is called \"{}\" and has mass {}", WATER.symbol(), WATER.name(), WATER.mass());

    let reaction = Reaction {
        lefthandside: &ReactionSide { compounds: &[
            ReactionCompound { amount: 2, molecule: &molecule_from_atom!(SODIUM) },
            ReactionCompound { amount: 1, molecule: &molecule_from_atom!(CHLORINE) }
        ]},

        righthandside: &ReactionSide { compounds: &[
            ReactionCompound { amount: 2, molecule: TABLE_SALT }
        ]}
    };

    if reaction.check_sides_equal() {
        println!("Reaction is valid");
    } else {
        println!("Reaction is invalid");
    }
}
