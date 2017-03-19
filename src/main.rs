extern crate feroxide;


use feroxide::{
    Ion,
    Molecule, MoleculeCompound,
    Container, ContainerCompound,
    ElemReaction, ReactionSide, ReactionCompound,
    RedoxReaction,
    Properties, Reaction
};

use feroxide::data_atoms::*;
use feroxide::data_molecules::*;
use feroxide::data_sep::*;


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

    // If you don't want to type a lot, you could also use strings
    let carbonic_acid = Molecule::from_string("H2CO3".to_owned()).unwrap();

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
    // H₂O + CO₂ ⇌ H₂CO₃
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


    // Redox is possible, but to save you from a lot of typing, I recommend using strings here
    let redox = RedoxReaction {
        oxidator: ElemReaction::<Ion>::from_string("F2 + 2e <> 2F;1-".to_owned()).unwrap(),
        reductor: ElemReaction::<Ion>::from_string("Fe <> Fe;3 + 3e".to_owned()).unwrap()
    };


    // Make sure it's valid
    assert!(redox.equalise());
    assert!(redox.is_valid());

    // Print the symbol version
    println!("{}", redox.symbol());

    // Print the SEP values
    println!("oxidator: {}", get_sep(&redox.oxidator).unwrap());
    println!("reductor: {}", get_sep(&redox.reductor).unwrap());
}
