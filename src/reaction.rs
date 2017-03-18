use molecule::*;
use trait_element::*;
use trait_properties::*;
use trait_reaction::*;
use types::*;

use std::collections::HashMap;


#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct ElemReaction<E> where E: Element {
    pub lhs: ReactionSide<E>,
    pub rhs: ReactionSide<E>,
    pub is_equilibrium: bool
}


#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct ReactionSide<E> where E: Element {
    pub compounds: Vec< ReactionCompound<E> >
}


#[derive(Debug, Eq, Copy, Clone, Hash)]
pub struct ReactionCompound<E> where E: Element {
    pub element: E,
    pub amount: u16
}



impl<E> ElemReaction<E> where E: Element {
    /// Get the sign of the equation ( → or ⇌ ), depending whether it is an equilibrium or not
    pub fn reaction_sign(&self) -> &str {
        if self.is_equilibrium {
            " ⇌ "
        } else {
            " → "
        }
    }


    /// Equalise the equation by changing the amount of moles necessary
    /// NOTE: This function is still a WIP!
    /// Returns true if it managed to equalise it, false otherwise
    pub fn equalise(&self) -> bool {
        println!("####    The equalise function is not yet ready.");


        let total_left = self.lhs.total_atoms();
        let total_right = self.rhs.total_atoms();

        // If both sides are already equal, do nothing
        if total_left == total_right {
            return true;
        }

        for (atom_number, l_amount) in total_left {
            let r_amount: u16;

            match total_right.get(&atom_number) {
                Some(&x) => { r_amount = x },
                None => { r_amount = 0 }
            }

            if r_amount == 0 {
                println!("It's impossible to make this reaction work: {}", self);
                return false;
            }

            if l_amount != r_amount {
                let difference = l_amount - r_amount;

                if difference > 0 {
                    // Increase right side
                    println!("We know what to do, but it's just not implemented yet.");
                } else {
                    // Increase left side
                    println!("We know what to do, but it's just not implemented yet.");
                }
            }
        }

        return false;
    }
}


impl<E> ReactionSide<E> where E: Element  {
    /// Calculate the total charge of this reaction side
    pub fn total_charge(&self) -> IonCharge {
        let mut total_charge = 0;

        for compound in self.compounds.iter() {
            if let Some(charge) = compound.element.get_charge() {
                total_charge += charge;
            }
        }

        return total_charge;
    }


    /// Calculate the energy this side has
    pub fn energy(&self) -> Energy {
        // NOTE: Temporary
        500.0 - (self.compounds.len() as f64) * 100.0
    }


    /// Calculate the total amount of atoms this side contains
    pub fn total_atoms(&self) -> HashMap<AtomNumber, u16> {
        let mut atoms: HashMap<AtomNumber, u16> = HashMap::new();

        // for molecule_compound in self.compounds:
        for reaction_compound in self.compounds.iter() {
            if let Some(ref molecule) = reaction_compound.element.get_molecule() {
                for molecule_compound in molecule.compounds.iter() {


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


impl<E> Reaction<E> for ElemReaction<E> where E: Element {
    fn is_valid(&self) -> bool {
        self.lhs.total_atoms() == self.rhs.total_atoms()
        && self.lhs.total_charge() == self.lhs.total_charge()
    }


    fn energy_cost(&self) -> Energy {
        self.rhs.energy() - self.lhs.energy()
    }


    fn elem_reaction(&self) -> ElemReaction<E> {
        self.clone()
    }
}


impl<E> ReactionCompound<E> where E: Element {
    pub fn stringify(&self) -> String {
        self.element.stringify()
    }
}


use std::ops::Add;
impl<E> Add for ReactionSide<E> where E: Element {
    type Output = ReactionSide<E>;

    fn add(mut self, mut rhs: ReactionSide<E>) -> ReactionSide<E> {
        self.compounds.append(&mut rhs.compounds);
        self
    }
}


impl<E> PartialEq for ReactionCompound<E> where E: Element {
    /// Two reactioncompounds are equal if their element is equal
    fn eq(&self, rhs: &ReactionCompound<E>) -> bool {
        self.element == rhs.element
    }
}


impl<E> Properties for ElemReaction<E> where E: Element  {
    fn symbol(&self) -> String {
        let mut symbol = String::new();

        symbol += &self.lhs.symbol();
        symbol += self.reaction_sign();
        symbol += &self.rhs.symbol();

        return symbol;
    }


    fn name(&self) -> String {
        let mut name = String::new();

        name += &self.lhs.name();
        name += self.reaction_sign();
        name += &self.rhs.name();

        return name;
    }


    fn mass(&self) -> AtomMass {
        panic!("Reactions do not have mass.");
    }
}


impl<E> Properties for ReactionSide<E> where E: Element  {
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

        for ref reaction_compound in self.compounds.iter() {
            mass += reaction_compound.mass();
        }

        return mass;
    }
}


impl<E> Properties for ReactionCompound<E> where E: Element  {
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


impl<E> Element for ReactionCompound<E> where E: Element {
    fn get_charge(&self) -> Option<IonCharge> {
        self.element.get_charge()
    }

    fn get_molecule(&self) -> Option<&Molecule> {
        self.element.get_molecule()
    }
}
