mod atom;
pub use atom::*;

mod molecule;
pub use molecule::*;

mod reaction;
pub use reaction::*;

mod ion;
pub use ion::*;

pub mod namings;
pub mod atoms;
pub mod molecules;
pub mod ions;

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
fn atoms_database_check() {
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
fn reaction_check() {
    use atoms::*;
    use molecules::*;

    let good_reaction = Reaction {
        lhs: &ReactionSide { compounds: &[
            ReactionCompound { amount: 2, molecule: &molecule_from_atom!(SODIUM) },
            ReactionCompound { amount: 1, molecule: &molecule_from_atom!(CHLORINE) }
        ]},

        rhs: &ReactionSide { compounds: &[
            ReactionCompound { amount: 2, molecule: TABLE_SALT }
        ]},

        is_equilibrium: false
    };


    let wrong_reaction_0 = Reaction {
        lhs: &ReactionSide { compounds: &[
            ReactionCompound { amount: 9, molecule: &molecule_from_atom!(SODIUM) },
            ReactionCompound { amount: 5, molecule: &molecule_from_atom!(CHLORINE) }
        ]},

        rhs: &ReactionSide { compounds: &[
            ReactionCompound { amount: 3, molecule: TABLE_SALT }
        ]},

        is_equilibrium: false
    };


    let wrong_reaction_1 = Reaction {
        lhs: &ReactionSide { compounds: &[
            ReactionCompound { amount: 1, molecule: &molecule_from_atom!(LITHIUM) }
        ]},

        rhs: &ReactionSide { compounds: &[
            ReactionCompound { amount: 3, molecule: &molecule_from_atom!(HYDROGEN) }
        ]},

        is_equilibrium: false
    };

    let equilibrium_reaction = Reaction {
        lhs: &ReactionSide { compounds: &[
            ReactionCompound { amount: 1, molecule: &molecule_from_atom!(HYDROGEN) }
        ]},

        rhs: &ReactionSide { compounds: &[
            ReactionCompound {
                amount: 2,
                molecule: &Molecule { compounds: &[
                    MoleculeCompound {
                        atom: HYDROGEN,
                        amount: 1
                    }
                ]}
            }
        ]},

        is_equilibrium: true
    };


    assert!(good_reaction.check_sides_equal());
    assert!(! wrong_reaction_0.check_sides_equal());
    assert!(! wrong_reaction_1.check_sides_equal());

    assert_eq!("2Na + Cl₂ → 2NaCl", good_reaction.to_string());
    assert_eq!("H₂ ←→ 2H", equilibrium_reaction.to_string());
}

#[test]
fn molecule_mass_calculation() {
    use molecules::*;

    assert_eq!(18.015, WATER.mass());
    assert_eq!(58.439774, TABLE_SALT.mass());
    assert_eq!(342.297, SUGAR.mass());
}



#[test]
#[should_panic]
fn panic_case_0() {
    namings::number_to_greek(250);
}
