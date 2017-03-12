extern crate feroxide;

use feroxide::*;
use feroxide::data_atoms::*;
use feroxide::data_molecules::*;
use feroxide::data_ions::*;
use feroxide::data_electron::*;

fn main() {
    println!("Hydrogen: {:?}", HYDROGEN);
    println!("Sugar: {}", SUGAR.symbol());
    println!("Sugar mass: {}", SUGAR.mass());
    println!("Sulphate: {}", SULPHATE.name());

    println!("{} is called \"{}\" and has mass {}", WATER.symbol(), WATER.name(), WATER.mass());

    let o4 = Ion {
        molecule: &Molecule { compounds: &[
            MoleculeCompound { atom: OXYGEN, amount: 4 }
        ]},
        data: Some(IonDataMap::charge(-2))
    };

    println!("{}  {}", o4.symbol(), o4.name());

    println!("sulphate: {} {}", SULPHATE.symbol(), SULPHATE.name());

    let ironiii = Ion {
        molecule: &molecule_from_atom!(IRON),
        data: Some(IonDataMap::charge(3))
    };

    println!("iron3: {} {}", ironiii.symbol(), ironiii.name());

    println!("The {} ({}) has a charge of {}",
        ELECTRON.name(), ELECTRON.symbol(), ELECTRON.get_charge().unwrap());
}
