use atom::*;
use ion::*;
use namings::*;

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
    pub ion: &'lifetime Ion<'lifetime>,
    pub amount: u8
}


impl<'lifetime> Reaction<'lifetime> {
    pub fn check_sides_equal(&self) -> bool {
        self.lhs.total_atoms() == self.rhs.total_atoms() && self.lhs.total_charge() == self.lhs.total_charge()
    }

    pub fn equalise(&self) -> bool {
        #![allow(unreachable_code)]
        panic!("The equalise function is not yet ready.");


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
                panic!("It's impossible to make this reaction work: {:?}", self);
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
    pub fn total_charge(&self) -> i8 {
        let mut total_charge = 0;

        for compound in self.compounds {
            if let Some(charge) = compound.ion.get_charge() {
                total_charge += charge;
            }
        }

        return total_charge;
    }

    pub fn total_atoms(&self) -> HashMap<AtomNumber, u8> {
        let mut atoms: HashMap<AtomNumber, u8> = HashMap::new();

        for reaction_compound in self.compounds {
            for molecule_compound in reaction_compound.ion.molecule.compounds {
                let atom_number = molecule_compound.atom.number;

                let old_amount;
                if let Some(&amount) = atoms.get(&atom_number) {
                    old_amount = amount;
                } else {
                    old_amount = 0;
                }

                let new_amount = old_amount + molecule_compound.amount * reaction_compound.amount;

                atoms.insert(atom_number, new_amount);
            }
        }

        return atoms;
    }
}

impl<'lifetime> Properties for Reaction<'lifetime> {
    fn symbol(&self) -> String {
        let mut symbol = String::new();

        symbol += &self.lhs.symbol();

        if self.is_equilibrium {
            symbol += " ↔ ";
        } else {
            symbol += " → ";
        }

        symbol += &self.rhs.symbol();

        return symbol;
    }

    fn name(&self) -> String {
        let mut name = String::new();

        name += &self.lhs.name();

        if self.is_equilibrium {
            name += " ↔ ";
        } else {
            name += " → ";
        }

        name += &self.rhs.symbol();

        return name;
    }

    fn mass(&self) -> AtomMass {
        panic!("Reaction does not have a mass.");
    }
}


impl<'lifetime> Properties for ReactionSide<'lifetime> {
    fn symbol(&self) -> String {
        let mut symbol = String::new();

        for (i, reaction_compound) in self.compounds.iter().enumerate() {
            if i > 0 {
                symbol += " + ";
            }

            symbol += &reaction_compound.symbol();
        }

        return symbol;
    }

    fn name(&self) -> String {
        let mut name = String::new();

        for (i, reaction_compound) in self.compounds.iter().enumerate() {
            if i > 0 {
                name += " + ";
            }

            name += &reaction_compound.name();
        }

        return name;
    }

    fn mass(&self) -> AtomMass {
        let mut mass = 0.0;

        for reaction_compound in self.compounds {
            mass += reaction_compound.mass();
        }

        return mass;
    }
}


impl<'lifetime> Properties for ReactionCompound<'lifetime> {
    fn symbol(&self) -> String {
        let mut symbol = String::new();

        if self.amount > 1 {
            symbol += &self.amount.to_string();
        }

        symbol += &self.ion.symbol();

        return symbol;
    }

    fn name(&self) -> String {
        let mut name = String::new();

        if self.amount > 1 {
            name += &number_to_greek(self.amount);
        }

        name += &self.ion.name();

        return name;
    }

    fn mass(&self) -> AtomMass {
        return (self.amount as AtomMass) * self.ion.mass();
    }
}
