use data_atoms::*;
use electron::*;
use ion::*;
use molecule::*;
use reaction::*;
use trait_element::*;
use types::*;

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::*;


// Reference: https://en.wikipedia.org/wiki/Standard_electrode_potential_(data_page)


/// Get the hash of a reaction
pub fn reaction_to_hash<E: Element>(reaction: &ElemReaction<E>) -> u64 {
    let mut s = DefaultHasher::new();
    reaction.hash(&mut s);
    s.finish()
}


/// Get the Standerd Electrode Potential (SEP) of a reaction
pub fn get_sep<E: Element>(elem_reaction: &ElemReaction<E>) -> Option<SEP> {
    if let Some(sep) = SEPMAP.get(& reaction_to_hash(&elem_reaction)) {
        Some(sep.clone())
    } else if let Some(sep) = SEPMAP.get(& reaction_to_hash(& elem_reaction.clone().swap())){
        Some(sep.clone())
    } else {
        None
    }
}


lazy_static! {
    pub static ref SEPMAP: HashMap<u64, SEP> = {
        let mut map = HashMap::new();

        {
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
                        element: ELECTRON.clone(),
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

            let hash = reaction_to_hash(&reaction);
            map.insert(hash, 0.0000);
        }


        {
            let reaction = ElemReaction {
                lhs: ReactionSide { compounds: vec! {
                    ReactionCompound {
                        element: Ion {
                            molecule: Molecule { compounds: vec! {
                                MoleculeCompound {
                                    atom: FLUORINE,
                                    amount: 2
                                }
                            }},

                            charge: Some(0)
                        },

                        amount: 1
                    },

                    ReactionCompound {
                        element: ELECTRON.clone(),
                        amount: 2
                    }
                }},

                rhs: ReactionSide { compounds: vec! {
                    ReactionCompound {
                        element: Ion {
                            molecule: Molecule { compounds: vec! {
                                MoleculeCompound {
                                    atom: FLUORINE,
                                    amount: 1
                                }
                            }},

                            charge: Some(-1)
                        },

                        amount: 2
                    }
                }},

                is_equilibrium: true
            };

            let hash = reaction_to_hash(&reaction);
            map.insert(hash, 2.87);
        }


        {
            let reaction = ElemReaction {
                lhs: ReactionSide { compounds: vec! {
                    ReactionCompound {
                        element: Ion {
                            molecule: Molecule { compounds: vec! {
                                MoleculeCompound {
                                    atom: IRON,
                                    amount: 1
                                }
                            }},

                            charge: Some(0)
                        },

                        amount: 1
                    }
                }},

                rhs: ReactionSide { compounds: vec! {
                    ReactionCompound {
                        element: Ion {
                            molecule: Molecule { compounds: vec! {
                                MoleculeCompound {
                                    atom: IRON,
                                    amount: 1
                                }
                            }},

                            charge: Some(3)
                        },

                        amount: 1
                    },

                    ReactionCompound {
                        element: ELECTRON.clone(),
                        amount: 3
                    }
                }},

                is_equilibrium: true
            };

            let hash = reaction_to_hash(&reaction);
            map.insert(hash, -0.04);
        }


        map
    };
}
