use properties::*;
use element::*;

use namings::*;
use types::*;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Reaction<'lifetime, T: 'lifetime> where T: Element {
    pub lhs: &'lifetime ReactionSide<'lifetime, T>,
    pub rhs: &'lifetime ReactionSide<'lifetime, T>,
    pub is_equilibrium: bool
}

#[derive(Debug)]
pub struct ReactionSide<'lifetime, T: 'lifetime> where T: Element {
    pub compounds: &'lifetime [ ReactionCompound<'lifetime, T> ]
}

#[derive(Debug)]
pub struct ReactionCompound<'lifetime, T: 'lifetime> where T: Element {
    pub element: &'lifetime T,
    pub amount: u8
}


impl<'lifetime, T> Reaction<'lifetime, T> where T: Element {
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

impl<'lifetime, T> ReactionSide<'lifetime, T> where T: Element  {
    pub fn total_charge(&self) -> IonCharge {
        let mut total_charge = 0;

        for compound in self.compounds {
            if let Some(charge) = compound.element.get_charge() {
                total_charge += charge;
            }
        }

        return total_charge;
    }

    pub fn total_atoms(&self) -> HashMap<AtomNumber, u8> {
        let mut atoms: HashMap<AtomNumber, u8> = HashMap::new();

        for reaction_compound in self.compounds {
            for molecule_compound in reaction_compound.element.get_molecule().compounds {
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

impl<'lifetime, T> Properties for Reaction<'lifetime, T> where T: Element  {
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


impl<'lifetime, T> Properties for ReactionSide<'lifetime, T> where T: Element  {
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


impl<'lifetime, T> Properties for ReactionCompound<'lifetime, T> where T: Element  {
    fn symbol(&self) -> String {
        let mut symbol = String::new();

        if self.amount > 1 {
            symbol += &self.amount.to_string();
        }

        symbol += &self.element.symbol();

        return symbol;
    }

    fn name(&self) -> String {
        let mut name = String::new();

        if self.amount > 1 {
            name += &number_to_greek(self.amount);
        }

        name += &self.element.name();

        return name;
    }

    fn mass(&self) -> AtomMass {
        return (self.amount as AtomMass) * self.element.mass();
    }
}
