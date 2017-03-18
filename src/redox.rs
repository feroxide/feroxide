use data_atoms::*;
use ion::*;
use electron::*;
use molecule::*;
use reaction::*;
use trait_element::*;
use trait_properties::*;
use trait_reaction::*;
use types::*;

use std::collections::HashMap;


macro_rules! molecule_from_atom {
    ($atom:expr) => (
        Molecule { compounds: vec! { MoleculeCompound::from_atom($atom) } }
    )
}

macro_rules! ion_from_molecule {
    ($molecule:expr) => (
        Ion { molecule: $molecule, charge: Some(0) }
    )
}

macro_rules! ion_from_atom {
    ($atom:expr) => (
        ion_from_molecule!(molecule_from_atom!($atom))
    )
}


#[derive(Debug, Eq, PartialEq, Clone)]
pub struct RedoxReaction<E: Element> {
    pub reductor: ElemReaction<E>,
    pub oxidator: ElemReaction<E>
}


#[allow(non_snake_case)]
// NOTE: This is not efficient AT ALL, but Rust won't let me make a const hashmap
// NOTE: Using Strings is also not very efficient, but Rust is getting annoying
pub fn SEPMAP() -> HashMap<String, SEP> {
    let mut map: HashMap<String, SEP> = HashMap::new();

    // Using data from https://en.wikipedia.org/wiki/Standard_electrode_potential_(data_page)

    let reaction = ElemReaction {
        lhs: ReactionSide { compounds: vec! {
            ReactionCompound {
                element: Ion {
                    molecule: Molecule {
                        compounds: vec! {
                            MoleculeCompound {
                                atom: HYDROGEN,
                                amount: 1
                            }
                        }
                    },
                    charge: Some(1)
                },
                amount: 2
            },

            ReactionCompound {
                element: ELECTRON(),
                amount: 2
            }
        }},

        rhs: ReactionSide { compounds: vec! {
            ReactionCompound {
                element: ion_from_atom!(HYDROGEN),
                amount: 1
            }
        }},

        is_equilibrium: true
    };

    let reaction_string = reaction.symbol();

    println!("{}", reaction_string);

    map.insert(reaction_string, 0.0000);

    map
}


pub fn get_sep<E>(elem_reaction: &ElemReaction<E>) -> Option<SEP> where E: Element {
    let sepmap = SEPMAP();

    let reaction_string = elem_reaction.symbol();

    if let Some(sep) = sepmap.get(&reaction_string) {
        Some(sep.clone())
    } else {
        None
    }
}


impl<E> Reaction<E> for RedoxReaction<E> where E: Element {
    fn is_valid(&self) -> bool {
        // oxidator > reductor
        get_sep(&self.oxidator) > get_sep(&self.reductor) && self.elem_reaction().is_valid()
    }


    fn energy_cost(&self) -> Energy {
        self.reductor.energy_cost() + self.oxidator.energy_cost()
    }


    fn elem_reaction(&self) -> ElemReaction<E> {
        ElemReaction {
            lhs: self.reductor.lhs.clone(),
            rhs: self.reductor.rhs.clone(),

            is_equilibrium: true
        }
    }
}


impl<E: Element> Properties for RedoxReaction<E> {
    fn symbol(&self) -> String {
        let mut symbol = String::new();

        symbol += "oxidator: ";
        symbol += &self.oxidator.symbol();
        symbol += "\n";
        symbol += "reductor: ";
        symbol += &self.reductor.symbol();

        return symbol;
    }

    fn name(&self) -> String {
        let mut name = String::new();

        name += "oxidator: ";
        name += &self.oxidator.name();
        name += "\n";
        name += "reductor: ";
        name += &self.reductor.name();

        return name;
    }

    fn mass(&self) -> AtomMass {
        // law of conservation of mass
        0.0
    }
}


impl<E: Element> RedoxReaction<E> {
    pub fn is_valid(&self) -> bool {
        true
    }
}
