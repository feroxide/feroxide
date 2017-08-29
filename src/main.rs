#[macro_use(ion_from_molecule)]
extern crate feroxide;


use feroxide::*;
use feroxide::data_atoms::*;
use feroxide::data_molecules::*;
use feroxide::data_sep::*;


fn main() {
    // You can create digital molecules with ease on two ways:
    // ... the easy way
    let carbondioxide = Molecule::from_string("CO2".to_owned()).unwrap();

    // ... and the fast way
    let carbonmonoxide = Molecule {
        compounds: vec![
            MoleculeCompound {
                atom: CARBON,
                amount: 1,
            },
            MoleculeCompound {
                atom: OXYGEN,
                amount: 1,
            },
        ],
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

    // Throw a bunch of molecules together in a container with a bit of energy
    let mut container = Container {
        contents: vec![
            ContainerCompound {
                element: ion_from_molecule!(carbonmonoxide.clone()),
                moles: Moles::from(1000.0),
            },
            ContainerCompound {
                element: ion_from_molecule!(WATER.clone()),
                moles: Moles::from(1000.0),
            },
            ContainerCompound {
                element: ion_from_atom!(OXYGEN.clone()),
                moles: Moles::from(1000.0),
            },
        ],

        available_energy: Energy::from(100_000f64), // in Joules
    };

    // Specify the reaction that will occur
    // H₂O + CO₂ ⇌ H₂CO₃
    let reaction = ElemReaction {
        lhs: ReactionSide {
            compounds: vec![
                ReactionCompound {
                    element: ion_from_atom!(OXYGEN.clone()),
                    amount: 1,
                },
                ReactionCompound {
                    element: ion_from_molecule!(carbonmonoxide.clone()),
                    amount: 2,
                },
            ],
        },

        rhs: ReactionSide {
            compounds: vec![
                ReactionCompound {
                    element: ion_from_molecule!(carbondioxide.clone()),
                    amount: 2,
                },
            ],
        },

        is_equilibrium: true,
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
        println!("[{:>2}] Contents: {}", i + 1, container.to_string());
    }


    // Redox reactions are also possible
    let redox = RedoxReaction {
        oxidator: ElemReaction::<Ion>::ion_from_string("F2 + 2e <> 2F;1-".to_owned()).unwrap(),
        reductor: ElemReaction::<Ion>::ion_from_string("Fe <> Fe;3 + 3e".to_owned()).unwrap(),
    };


    // Make sure it's valid
    assert!(redox.equalise());
    assert!(redox.is_valid());

    // Print the symbol representation
    println!("{}", redox.symbol());

    // Print the SEP values
    println!("oxidator: {}", get_sep(&redox.oxidator).unwrap());
    println!("reductor: {}", get_sep(&redox.reductor).unwrap());
}
