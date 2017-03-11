use atom::*;
use molecule::*;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Reaction<'lifetime> {
    pub lefthandside: &'lifetime ReactionSide<'lifetime>,
    pub righthandside: &'lifetime ReactionSide<'lifetime>
}

#[derive(Debug)]
pub struct ReactionSide<'lifetime> {
    pub compounds: &'lifetime [ ReactionCompound<'lifetime> ]
}

#[derive(Debug)]
pub struct ReactionCompound<'lifetime> {
    pub molecule: &'lifetime Molecule<'lifetime>,
    pub amount: u8
}


impl<'lifetime> Reaction<'lifetime> {
    pub fn check_sides_equal(&self) -> bool {
        let lhs_total = self.lefthandside.total_atoms();
        let rhs_total = self.righthandside.total_atoms();

        lhs_total == rhs_total
    }
}

impl<'lifetime> ReactionSide<'lifetime> {
    pub fn total_atoms(&self) -> HashMap<AtomNumber, u8> {
        let mut atoms: HashMap<AtomNumber, u8> = HashMap::new();

        for compound in self.compounds {
            for molcompound in compound.molecule.compounds {
                let number = molcompound.atom.number;

                let old_amount;
                match atoms.get(&number) {
                    Some(&amount) => { old_amount = amount },
                    None => { old_amount = 0 }
                };

                let new_amount = old_amount + molcompound.amount * compound.amount;

                atoms.insert(number, new_amount);
            }
        }

        return atoms;
    }
}
