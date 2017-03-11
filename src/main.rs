extern crate feroxide;

use feroxide::*;
use feroxide::atoms::*;
use feroxide::molecules::*;
use feroxide::ions::*;

macro_rules! molecule_from_atom {
    ($atom:expr) => (
        Molecule { compounds: &[ MoleculeCompound::from_atom($atom) ] }
    )
}

fn main() {
    println!("Hydrogen: {:?}", HYDROGEN);
    println!("Table salt: {:?}", TABLE_SALT);
    println!("Table salt mass: {:?}", TABLE_SALT.mass());

    println!("{} is called \"{}\" and has mass {}", WATER.symbol(), WATER.name(), WATER.mass());

    react();

    println!("sulphate: {}", SULPHATE.symbol());
}


fn react() {
    // 2Na + Cl2 --> 2NaCl
    let reaction = Reaction {
        lhs: &ReactionSide { compounds: &[
            ReactionCompound { amount: 2, molecule: &molecule_from_atom!(SODIUM) },
            ReactionCompound { amount: 1, molecule: &molecule_from_atom!(CHLORINE) }
        ]},

        rhs: &ReactionSide { compounds: &[
            ReactionCompound { amount: 2, molecule: TABLE_SALT }
        ]},

        is_equilibrium: false
    };

    if reaction.check_sides_equal() {
        println!("Reaction {} is valid", reaction.to_string());
    } else {
        println!("Reaction {} is invalid", reaction.to_string());
    }
}
