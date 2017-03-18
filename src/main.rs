extern crate feroxide;


use feroxide::*;
use feroxide::data_atoms::*;
use feroxide::data_molecules::*;


fn main() {
    // You can create digital molecules with ease
    let carbondioxide = Molecule {
        compounds: vec! {
            MoleculeCompound { atom: CARBON, amount: 1 },
            MoleculeCompound { atom: OXYGEN, amount: 2 }
        }
    };

    // Of which you can generate the name
    let _name = carbondioxide.name();

    // ... or the symbol
    let symbol = carbondioxide.symbol();

    // You can calculate the mass per mole
    let mass_per_mole = carbondioxide.mass();

    // Multiply that with your amount of moles
    let weight = mass_per_mole * 10.0;

    // To get your data
    println!("10 moles of {} weigh {} gram(s).", symbol, weight);


    // Make some more molecules, just for fun
    let carbonic_acid = Molecule {
        compounds: vec! {
            MoleculeCompound { atom: HYDROGEN, amount: 2 },
            MoleculeCompound { atom: CARBON, amount: 1 },
            MoleculeCompound { atom: OXYGEN, amount: 3 }
        }
    };

    // Throw a bunch of molecules together in a container with a bit of energy
    let mut container = Container {
        contents: vec! {
            ContainerCompound {
                element: carbondioxide.clone(),
                moles: 1000.0
            },

            ContainerCompound {
                element: WATER.clone(),
                moles: 1000.0
            }
        },

        available_energy: 100_000f64 // in Joules
    };

    // Specify the reaction that will occur
    // H₂O + CO₂ → H₂CO₃
    let reaction = ElemReaction {
        lhs: ReactionSide {
            compounds: vec! {
                ReactionCompound { element: WATER.clone(), amount: 1 },
                ReactionCompound { element: carbondioxide.clone(), amount: 1 }
            }
        },

        rhs: ReactionSide {
            compounds: vec! {
                ReactionCompound { element: carbonic_acid.clone(), amount: 1 }
            }
        },

        is_equilibrium: true
    };

    // Make sure the reaction is valid
    assert!(reaction.equalise());
    assert!(reaction.is_valid());

    // Print the reaction in names
    println!("{}", reaction.name());

    // ... or in symbols (the default)
    println!("{}", reaction.symbol());


    // Print the contents of the container at the start
    println!("Contents: {}", container);


    // Run the reaction 10 times
    for i in 0..10 {
        // Run the reaction on the container
        container.react(&reaction);

        // Show what's left
        println!("[{:>2}] Contents: {}", i+1, container.to_string());
    }



    // Redox is possible, but it's quite a hassle to type
    let redox = RedoxReaction {
        oxidator: ElemReaction {
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
        },

        reductor: ElemReaction {
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
        }
    };


    // Make sure it's valid
    assert!(redox.equalise());
    assert!(redox.is_valid());

    // Print the symbol version
    println!("{}", redox.symbol());

    // Print the name version
    println!("{}", redox.name());
}
