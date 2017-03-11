use ion::*;
use molecule::*;
use atoms::*;

pub fn calculate_charge(compounds: &'static [IonCompound<'static>]) -> i8 {
    let mut total_charge: i8 = 0;

    for compound in compounds {
        total_charge += (compound.amount as i8) * compound.charge;
    }

    return total_charge;
}

const SULPHATE_COMPOUNDS: &'static [IonCompound<'static>] = &[
    IonCompound {
        molecule: Molecule { compounds: &[
            MoleculeCompound {
                atom: SULFUR,
                amount: 1
            },
            MoleculeCompound {
                atom: OXYGEN,
                amount: 4
            }
        ]},
        amount: 1,
        charge: -2
    }
];

/*
const SULPHATE_CHARGE: i8 = calculate_charge(SULPHATE_COMPOUNDS);
*/
const SULPHATE_CHARGE: i8 = -2;


pub const SULPHATE: &'static Ion<'static> = &Ion {
    compounds: SULPHATE_COMPOUNDS, total_charge: SULPHATE_CHARGE };
