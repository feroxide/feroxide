extern crate feroxide;

use feroxide::*;
use feroxide::atoms::*;
use feroxide::molecules::*;
// use feroxide::ions::*;

fn main() {
    println!("Hydrogen: {:?}", HYDROGEN);
    println!("Table salt: {}", TABLE_SALT.name());
    println!("Table salt mass: {}", TABLE_SALT.mass());

    println!("{} is called \"{}\" and has mass {}", WATER.symbol(), WATER.name(), WATER.mass());

    react();

    let o4 = Ion { molecule: &Molecule { compounds: &[
        MoleculeCompound { atom: OXYGEN, amount: 4 }
    ]}, data: Some(IonDataMap::charge(-2)) };

    println!("{}  {}", o4.symbol(), o4.name());

    /*
    println!("sulphate: {} {}", SULPHATE.symbol(), SULPHATE.name());

    let ironiii = Ion { compounds: &[
        IonCompound { molecule: &molecule_from_atom!(IRON), amount: 1, charge: 3 }
    ], total_charge: 3 };

    println!("iron3: {} {}", ironiii.symbol(), ironiii.name());

    println!("The {} ({}) has a charge of {}", ELECTRON.name(), ELECTRON.symbol(), ELECTRON.total_charge);
    */
}


fn react() {
    // 2Na + Cl2 --> 2NaCl
    let reaction = Reaction {
        lhs: &ReactionSide { compounds: &[
            ReactionCompound { amount: 2, ion: &ion_from_atom!(SODIUM) },
            ReactionCompound { amount: 1, ion: &ion_from_atom!(CHLORINE) }
        ]},

        rhs: &ReactionSide { compounds: &[
            ReactionCompound { amount: 2, ion: &ion_from_molecule!(TABLE_SALT) }
        ]},

        is_equilibrium: false
    };

    if reaction.check_sides_equal() {
        println!("Reaction {} is valid", reaction.to_string());
    } else {
        println!("Reaction {} is invalid", reaction.to_string());
    }
}
