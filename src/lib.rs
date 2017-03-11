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
fn greek_namings() {
    assert_eq!("di", namings::number_to_greek(2));
}

#[test]
fn roman_namings() {
    assert_eq!("0", namings::number_to_roman(0));
    assert_eq!("IV", namings::number_to_roman(4));
    assert_eq!("-III", namings::number_to_roman(-3));
    assert_eq!("VII", namings::number_to_roman(7));
    assert_eq!("-VII", namings::number_to_roman(-7));
}
