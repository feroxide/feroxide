use atom::*;
use molecule::*;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Reaction<'lifetime> {
    pub lhs: &'lifetime ReactionSide<'lifetime>,
    pub rhs: &'lifetime ReactionSide<'lifetime>,
    pub is_equilibrium: bool
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
        self.lhs.total_atoms() == self.rhs.total_atoms()
    }

    pub fn to_string(&self) -> String {
        let mut string = String::new();

        string += &self.lhs.to_string();

        if self.is_equilibrium {
            string += " ↔ ";
        } else {
            string += " → ";
        }

        string += &self.rhs.to_string();

        return string;
    }

    pub fn equalise(&self) -> bool {
        let total_left = self.lhs.total_atoms();
        let total_right = self.rhs.total_atoms();

        if total_left == total_right {
            return true;
        }

        for (atom_number, l_amount) in total_left {
            let r_amount: u8;

            match total_right.get(&atom_number) {
                Some(x) => { r_amount = x.to_owned() },
                None => { r_amount = 0 }
            }

            if r_amount == 0 {
                panic!("It's impossible to make this reaction work: {}", self.to_string());
            }

            if l_amount != r_amount {
                let difference = l_amount - r_amount;

                if difference > 0 {

                }
            }
        }

        return true;
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

    pub fn to_string(&self) -> String {
        let mut string = String::new();

        let len = self.compounds.len();
        for (i, compound) in self.compounds.iter().enumerate() {
            string += &compound.to_string();

            if i < len - 1 {
                string += " + ";
            }
        }

        return string;
    }
}

impl<'lifetime> ReactionCompound<'lifetime> {
    pub fn to_string(&self) -> String {
        let mut string = String::new();

        if self.amount > 1 {
            string += &self.amount.to_string();
        }

        string += &self.molecule.symbol();

        return string;
    }
}
