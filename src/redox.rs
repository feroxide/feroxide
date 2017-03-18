// use data_atoms::*;
// use ion::*;
// use electron::*;
// use molecule::*;
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
pub struct Redox<E: Element> {
    pub reductor: ElemReaction<E>,
    pub oxidator: ElemReaction<E>
}


#[allow(non_snake_case)]
// NOTE: This is not efficient AT ALL, but Rust won't let me make a const hashmap
pub fn SEPMAP<E>() -> HashMap<ElemReaction<E>, SEP> where E: Element {
    let mut map: HashMap<ElemReaction<E>, SEP> = HashMap::new();

    // Using data from https://en.wikipedia.org/wiki/Standard_electrode_potential_(data_page)

    map.insert(ElemReaction {
        lhs: ReactionSide { compounds: vec! {
            /*
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
            */
        }},

        rhs: ReactionSide { compounds: vec! {
            /*
            ReactionCompound {
                element: ion_from_atom!(HYDROGEN),
                amount: 1
            }
            */
        }},

        is_equilibrium: true
    }, 0.0000);


    map
}


pub fn get_sep<E>(elem_reaction: &ElemReaction<E>) -> SEP where E: Element {
    let sepmap = SEPMAP();

    sepmap.get(elem_reaction).unwrap().clone()
}


impl<E> Reaction<E> for Redox<E> where E: Element {
    fn is_valid(&self) -> bool {
        get_sep(&self.reductor) < get_sep(&self.oxidator) && self.elem_reaction().is_valid()
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


impl<E: Element> Properties for Redox<E> {
    fn symbol(&self) -> String {
        "".to_owned()
    }

    fn name(&self) -> String {
        "".to_owned()
    }

    fn mass(&self) -> AtomMass {
        0.0
    }
}


impl<E: Element> Redox<E> {
    pub fn is_valid(&self) -> bool {
        true
    }
}
