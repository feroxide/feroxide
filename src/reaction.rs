use properties::*;
use element::*;

use types::*;

use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub struct Reaction<'lifetime, T: 'lifetime> where T: Element {
    pub lhs: ReactionSide<'lifetime, T>,
    pub rhs: ReactionSide<'lifetime, T>,
    pub is_equilibrium: bool
}

#[derive(Debug, Copy, Clone)]
pub struct ReactionSide<'lifetime, T: 'lifetime> where T: Element {
    pub compounds: &'lifetime [ ReactionCompound<T> ]
}

#[derive(Debug, Eq, Copy, Clone)]
pub struct ReactionCompound<T> where T: Element {
    pub element: T,
    pub amount: u16
}



impl<T> PartialEq for ReactionCompound<T> where T: Element {
    fn eq(&self, rhs: &ReactionCompound<T>) -> bool {
        self.element == rhs.element
    }
}


impl<'lifetime, T> Reaction<'lifetime, T> where T: Element {
    pub fn is_valid(&self) -> bool {
        self.lhs.total_atoms() == self.rhs.total_atoms() && self.lhs.total_charge() == self.lhs.total_charge()
    }

    pub fn energy_cost(&self) -> Energy {
        self.rhs.energy() - self.lhs.energy()
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
            let r_amount: u16;

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

    pub fn energy(&self) -> Energy {
        // NOTE: Temporary
        500.0 - (self.compounds.len() as f64) * 100.0
    }


    pub fn total_atoms(&self) -> HashMap<AtomNumber, u16> {
        let mut atoms: HashMap<AtomNumber, u16> = HashMap::new();

        for reaction_compound in self.compounds {
            if let Some(ref molecule) = reaction_compound.element.get_molecule() {
                for molecule_compound in molecule.compounds {
                    let atom_number = molecule_compound.atom.number;

                    let mut amount;
                    if let Some(&old_amount) = atoms.get(&atom_number) {
                        amount = old_amount;
                    } else {
                        amount = 0;
                    }

                    amount += (molecule_compound.amount as u16) * reaction_compound.amount;

                    atoms.insert(atom_number, amount);
                }
            }
        }

        return atoms;
    }
}

impl<T> ReactionCompound<T> where T: Element {
    pub fn subtract_amount(&mut self, x: u16) {
        self.amount -= x;
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

        name += &self.rhs.name();

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


impl<T> Properties for ReactionCompound<T> where T: Element  {
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
            name += &self.amount.to_string();
        }

        name += &self.element.name();

        return name;
    }

    fn mass(&self) -> AtomMass {
        return (self.amount as AtomMass) * self.element.mass();
    }
}
