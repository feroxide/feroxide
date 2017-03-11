mod atom;
pub use atom::Atom;

pub mod atoms;

mod molecule;
pub use molecule::{ MoleculeCompound, Molecule };

pub mod namings;


#[test]
fn database_correct() {
    assert_eq!(1, atoms::HYDROGEN.atom_number);
    assert_eq!("oxygen", atoms::OXYGEN.name);
    assert_eq!("He", atoms::HELIUM.symbol);
    assert_eq!(35.45, atoms::CHLORINE.mass);

    assert_eq!("aluminium", atoms::ALUMINIUM.name);
}

#[test]
fn molecule_mass() {
    use atoms::*;

    let water = Molecule {
        compounds: &vec! {
            MoleculeCompound { atom: HYDROGEN, amount: 2 },
            MoleculeCompound { atom: OXYGEN, amount: 1 }
        }
    };

    let sugar = Molecule {
        compounds: &vec! {
            MoleculeCompound { atom: CARBON, amount: 12 },
            MoleculeCompound { atom: HYDROGEN, amount: 22 },
            MoleculeCompound { atom: OXYGEN, amount: 11 }
        }
    };


    assert_eq!(18.015, water.mass());
    assert_eq!(342.297, sugar.mass());
}

#[test]
fn greek_namings() {
    assert_eq!("di", namings::number_to_greek(2));
    assert_eq!("undeca", namings::number_to_greek(11));
    assert_eq!("dodeca", namings::number_to_greek(12));
    assert_eq!("tetradeca", namings::number_to_greek(14));
    assert_eq!("icosa", namings::number_to_greek(20));
    assert_eq!("henicosa", namings::number_to_greek(21));
    assert_eq!("tricosa", namings::number_to_greek(23));
    assert_eq!("triaconta", namings::number_to_greek(30));
    assert_eq!("pentatriaconta", namings::number_to_greek(35));
    assert_eq!("pentaconta", namings::number_to_greek(50));
    assert_eq!("dohexaconta", namings::number_to_greek(62));
    assert_eq!("nonanonaconta", namings::number_to_greek(99));
}

#[test]
fn roman_namings() {
    assert_eq!("0", namings::number_to_roman(0));
    assert_eq!("IV", namings::number_to_roman(4));
    assert_eq!("-III", namings::number_to_roman(-3));
    assert_eq!("VII", namings::number_to_roman(7));
    assert_eq!("-VII", namings::number_to_roman(-7));
    assert_eq!("XI", namings::number_to_roman(11));
    assert_eq!("XIV", namings::number_to_roman(14));
    assert_eq!("-XVI", namings::number_to_roman(-16));
}
