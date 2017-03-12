mod element;
mod properties;
pub use element::*;
pub use properties::*;

mod atom;
mod molecule;
mod ion;

pub use atom::*;
pub use molecule::*;
pub use ion::*;

mod reaction;
pub use reaction::*;

mod namings;
pub use namings::*;

mod types;
pub use types::*;

pub mod data_atoms;
pub mod data_molecules;
pub mod data_ions;

#[macro_export]
macro_rules! molecule_from_atom {
    ($atom:expr) => (
        Molecule { compounds: &[ MoleculeCompound::from_atom($atom) ] }
    )
}

#[macro_export]
macro_rules! ion_from_molecule {
    ($molecule:expr) => (
        Ion { molecule: $molecule, data: Some(IonDataMap::charge(0)) }
    )
}

#[macro_export]
macro_rules! ion_from_atom {
    ($atom:expr) => (
        ion_from_molecule!(&molecule_from_atom!($atom))
    )
}


// tests \\


#[test]
fn ion_notation_check() {
    use data_atoms::*;

    // O₄²⁻  tetraoxygen(-II)
    let o4 = Ion {
        molecule: &Molecule { compounds: &[
            MoleculeCompound { atom: OXYGEN, amount: 4 }
        ]},
        data: Some(IonDataMap::charge(-2))
    };


    assert_eq!("O₄²⁻", o4.symbol());
    assert_eq!("tetraoxygen(-II)", o4.name());
}


#[test]
fn ion_charge_calculation() {
    use data_ions::*;

    assert_eq!(-2, SULPHATE.get_charge().unwrap());
    assert_eq!(-1, HYDROXIDE.get_charge().unwrap());
    assert_eq!(1, AMMONIUM.get_charge().unwrap());
}

#[test]
fn electron_data() {
    use data_ions::*;

    assert_eq!(-1, ELECTRON.get_charge().unwrap());
    assert_eq!("e⁻", ELECTRON.symbol());
}


#[test]
fn diatomic_check() {
    use data_atoms::*;

    assert_eq!(2, MoleculeCompound::from_atom(HYDROGEN).amount);
    assert_eq!(2, MoleculeCompound::from_atom(NITROGEN).amount);
    assert_eq!(2, MoleculeCompound::from_atom(OXYGEN).amount);
    assert_eq!(2, MoleculeCompound::from_atom(FLUORINE).amount);
    assert_eq!(2, MoleculeCompound::from_atom(CHLORINE).amount);
    assert_eq!(2, MoleculeCompound::from_atom(BROMINE).amount);
    assert_eq!(2, MoleculeCompound::from_atom(IODINE).amount);
    assert_eq!(1, MoleculeCompound::from_atom(CARBON).amount);
    assert_eq!(1, MoleculeCompound::from_atom(LITHIUM).amount);
    assert_eq!(1, MoleculeCompound::from_atom(SULFUR).amount);
}

#[test]
fn atoms_database_check() {
    use data_atoms::*;

    assert_eq!(1, HYDROGEN.number);
    assert_eq!("oxygen", OXYGEN.name);
    assert_eq!("He", HELIUM.symbol);
    assert_eq!(35.45, CHLORINE.mass);

    assert!(HYDROGEN.is_diatomic);
    assert!(OXYGEN.is_diatomic);
    assert!(! CARBON.is_diatomic);
    assert!(! SULFUR.is_diatomic);

    // Very important
    assert_eq!("aluminium", ALUMINIUM.name);
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
    use data_atoms::*;
    use data_molecules::*;

    let good_reaction = Reaction {
        lhs: &ReactionSide { compounds: &[
            ReactionCompound { amount: 1, element: &molecule_from_atom!(CARBON) },
            ReactionCompound { amount: 1, element: &molecule_from_atom!(OXYGEN) }
        ]},

        rhs: &ReactionSide { compounds: &[
            ReactionCompound { amount: 1, element: CO2 }
        ]},

        is_equilibrium: false
    };


    let wrong_reaction_0 = Reaction {
        lhs: &ReactionSide { compounds: &[
            ReactionCompound { amount: 9, element: &molecule_from_atom!(CARBON) },
            ReactionCompound { amount: 5, element: &molecule_from_atom!(OXYGEN) }
        ]},

        rhs: &ReactionSide { compounds: &[
            ReactionCompound { amount: 3, element: CO2 }
        ]},

        is_equilibrium: false
    };


    let wrong_reaction_1 = Reaction {
        lhs: &ReactionSide { compounds: &[
            ReactionCompound { amount: 1, element: &molecule_from_atom!(LITHIUM) }
        ]},

        rhs: &ReactionSide { compounds: &[
            ReactionCompound { amount: 3, element: &molecule_from_atom!(HYDROGEN) }
        ]},

        is_equilibrium: false
    };

    let equilibrium_reaction = Reaction {
        lhs: &ReactionSide { compounds: &[
            ReactionCompound { amount: 1, element: &molecule_from_atom!(HYDROGEN) }
        ]},

        rhs: &ReactionSide { compounds: &[
            ReactionCompound {
                amount: 2,
                element: &Molecule { compounds: &[
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

    // TODO: Ammonium reactions

    assert_eq!("C + O₂ → CO₂", good_reaction.to_string());
    assert_eq!("H₂ ↔ 2H", equilibrium_reaction.to_string());
}


#[test]
#[should_panic]
fn equalise() {
    use data_atoms::*;
    use data_molecules::*;

    let water_reaction = Reaction {
        lhs: &ReactionSide { compounds: &[
            ReactionCompound { element: &molecule_from_atom!(HYDROGEN), amount: 0 },
            ReactionCompound { element: &molecule_from_atom!(OXYGEN), amount: 0 }
        ]},

        rhs: &ReactionSide { compounds: &[
            ReactionCompound { element: WATER, amount: 0 }
        ]},

        is_equilibrium: false
    };

    water_reaction.equalise();
}


#[test]
fn compare_similiar_elements_only() {
    use data_molecules::*;
    use data_ions::*;

    let _ = Reaction {
        lhs: &ReactionSide { compounds: &[
            ReactionCompound { element: WATER, amount: 1 }
        ]},

        rhs: &ReactionSide { compounds: &[
            ReactionCompound { element: CO2, amount: 1 }
        ]},

        is_equilibrium: true
    };

    let _ = Reaction {
        lhs: &ReactionSide { compounds: &[
            ReactionCompound { element: ELECTRON, amount: 1 }
        ]},

        rhs: &ReactionSide { compounds: &[
            ReactionCompound { element: SULPHATE, amount: 1 }
        ]},

        is_equilibrium: true
    };
}


#[test]
fn molecule_mass_calculation() {
    use data_molecules::*;

    assert_eq!(18.015, WATER.mass());
    assert_eq!(342.297, SUGAR.mass());
}



#[test]
#[should_panic]
fn panic_case_0() {
    namings::number_to_greek(250);
}
