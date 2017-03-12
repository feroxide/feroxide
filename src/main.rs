extern crate feroxide;

use feroxide::*;
use feroxide::data_molecules::*;
use feroxide::data_atoms::*;


fn main() {
    // You can create digital molecules with ease
    let carbondioxide = &Molecule {
        compounds: &[
            MoleculeCompound { atom: CARBON, amount: 1 },
            MoleculeCompound { atom: OXYGEN, amount: 2 }
        ]
    };

    // Generate a name
    let _name = carbondioxide.name();

    // ... or symbol
    let symbol = carbondioxide.symbol();

    // You can calculate the mass per mole
    let mass_per_mole = carbondioxide.mass();

    // Multiply that with your amount of moles
    let weight = mass_per_mole * 10.0;

    // To get your data
    println!("10 moles of {} weigh {} gram(s).", symbol, weight);


    // Make some more molecules which we'll use later
    let carbonic_acid = &Molecule {
        compounds: & [
            MoleculeCompound { atom: HYDROGEN, amount: 2 },
            MoleculeCompound { atom: CARBON, amount: 1 },
            MoleculeCompound { atom: OXYGEN, amount: 3 }
        ]
    };


    // You could also throw some molecules together in a container with a bit of energy
    let container = Container {
        contents: &[
            &ReactionCompound {
                element: carbondioxide,
                amount: 1000 // moles
            },

            &ReactionCompound {
                element: WATER,
                amount: 1000 // moles
            }
        ],

        available_energy: 100_000f64 // in Joules
    };

    // Then specify a reaction that will occur
    // H2O + CO2 --> H2CO3
    let reaction = &Reaction {
        lhs: &ReactionSide {
            compounds: &[
                &ReactionCompound { element: WATER, amount: 1 },
                &ReactionCompound { element: carbondioxide, amount: 1 }
            ]
        },

        rhs: &ReactionSide {
            compounds: &[
                &ReactionCompound { element: carbonic_acid, amount: 1 }
            ]
        },

        is_equilibrium: false
    };

    // Make sure the reaction is valid
    assert!(reaction.is_valid());

    // Print the reaction in names
    println!("{}", reaction.name());

    // Print the reaction in symbols
    println!("{}", reaction.symbol());

    // Print the energy cost
    println!("Energy cost: {}", reaction.energy_cost());

    // Run the reaction on container
    // container.react(reaction);

    println!("Energy left: {:?}", container.available_energy);
}
