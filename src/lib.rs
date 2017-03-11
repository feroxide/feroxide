mod atom;
pub use atom::*;

mod molecule;
pub use molecule::*;

mod reaction;
pub use reaction::*;

pub mod namings;
pub mod atoms;
pub mod molecules;

macro_rules! molecule_from_atom {
    ($atom:expr) => (
        Molecule { compounds: &[ MoleculeCompound::from_atom($atom) ] }
    )
}


#[test]
fn diatomic_check() {
    use atoms::*;

    assert_eq!(2, MoleculeCompound::from_atom(HYDROGEN).amount);
    assert_eq!(2, MoleculeCompound::from_atom(NITROGEN).amount);
    assert_eq!(2, MoleculeCompound::from_atom(OXYGEN).amount);
    assert_eq!(2, MoleculeCompound::from_atom(FLUORINE).amount);
    assert_eq!(2, MoleculeCompound::from_atom(CHLORINE).amount);
    assert_eq!(2, MoleculeCompound::from_atom(BROMINE).amount);
    assert_eq!(2, MoleculeCompound::from_atom(IODINE).amount);
    assert_eq!(1, MoleculeCompound::from_atom(CARBON).amount);
    assert_eq!(1, MoleculeCompound::from_atom(LITHIUM).amount);
}

#[test]
fn reaction_check() {
    use atoms::*;

    let sodium_molecule = &Molecule {
        compounds: &[ MoleculeCompound { atom: SODIUM, amount: 1 } ]
    };

    let chlorine_molecule = &Molecule {
        compounds: &[ MoleculeCompound { atom: CHLORINE, amount: 2 } ]
    };

    let lithium_molecule = &Molecule {
        compounds: &[ MoleculeCompound { atom: LITHIUM, amount: 1 } ]
    };

    let hydrogen_molecule = &Molecule {
        compounds: &[ MoleculeCompound { atom: HYDROGEN, amount: 2 } ]
    };


    let salt_molecule = &Molecule {
        compounds: &[
            MoleculeCompound { atom: CHLORINE, amount: 1 },
            MoleculeCompound { atom: SODIUM, amount: 1 }
        ]
    };

    let good_reaction = Reaction {
        lefthandside: &ReactionSide {
            compounds: &[
                ReactionCompound { amount: 2, molecule: sodium_molecule },
                ReactionCompound { amount: 1, molecule: chlorine_molecule }
            ]
        },

        righthandside: &ReactionSide {
            compounds: &[  ReactionCompound { amount: 2, molecule: salt_molecule }  ]
        }
    };

    let wrong_reaction_0 = Reaction {
        lefthandside: &ReactionSide {
            compounds: &[
                ReactionCompound { amount: 9, molecule: sodium_molecule },
                ReactionCompound { amount: 5, molecule: chlorine_molecule }
            ]
        },

        righthandside: &ReactionSide {
            compounds: &[  ReactionCompound { amount: 3, molecule: salt_molecule }  ]
        }
    };

    let wrong_reaction_1 = Reaction {
        lefthandside: &ReactionSide {
            compounds: &[
                ReactionCompound { amount: 1, molecule: lithium_molecule }
            ]
        },

        righthandside: &ReactionSide {
            compounds: &[
                ReactionCompound { amount: 3, molecule: hydrogen_molecule }
            ]
        }
    };


    assert!(good_reaction.check_sides_equal());
    assert!(! wrong_reaction_0.check_sides_equal());
    assert!(! wrong_reaction_1.check_sides_equal());
}


#[test]
fn atoms_database_correct() {
    use atoms::*;

    assert_eq!(1, HYDROGEN.number);
    assert_eq!("oxygen", OXYGEN.name);
    assert_eq!("He", HELIUM.symbol);
    assert_eq!(35.45, CHLORINE.mass);

    assert!(HYDROGEN.is_diatomic);
    assert!(OXYGEN.is_diatomic);
    assert!(! CARBON.is_diatomic);
    assert!(! SULFUR.is_diatomic);

    assert_eq!("aluminium", atoms::ALUMINIUM.name);
}

#[test]
fn molecule_mass_calculation() {
    use atoms::*;
    use molecules::*;

    let sugar = Molecule {
        compounds: &[
            MoleculeCompound { atom: CARBON, amount: 12 },
            MoleculeCompound { atom: HYDROGEN, amount: 22 },
            MoleculeCompound { atom: OXYGEN, amount: 11 }
        ]
    };


    assert_eq!(18.015, WATER.mass());
    assert_eq!(342.297, sugar.mass());
}

#[test]
fn greek_namings() {
    use namings::number_to_greek as to_greek;

    assert_eq!("di", to_greek(2));
    assert_eq!("undeca", to_greek(11));
    assert_eq!("dodeca", to_greek(12));
    assert_eq!("tetradeca", to_greek(14));
    assert_eq!("icosa", to_greek(20));
    assert_eq!("henicosa", to_greek(21));
    assert_eq!("tricosa", to_greek(23));
    assert_eq!("triaconta", to_greek(30));
    assert_eq!("pentatriaconta", to_greek(35));
    assert_eq!("pentaconta", to_greek(50));
    assert_eq!("dohexaconta", to_greek(62));
    assert_eq!("nonanonaconta", to_greek(99));
}

#[test]
fn roman_namings() {
    use namings::number_to_roman as to_roman;

    assert_eq!("0", to_roman(0));
    assert_eq!("IV", to_roman(4));
    assert_eq!("-III", to_roman(-3));
    assert_eq!("VII", to_roman(7));
    assert_eq!("-VII", to_roman(-7));
    assert_eq!("XI", to_roman(11));
    assert_eq!("XIV", to_roman(14));
    assert_eq!("-XVI", to_roman(-16));
}

#[test]
#[should_panic]
fn panic_case_0() {
    namings::number_to_greek(250);
}
