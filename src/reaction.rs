use ion::*;
use molecule::*;
use trait_element::*;
use trait_properties::*;
use trait_reaction::*;
use types::*;

use std::collections::HashMap;
use std::ops::*;


#[derive(Debug, Eq, PartialEq, Clone, Hash)]
/// An elementair reaction (not containing ions)
pub struct ElemReaction<E: Element> {
    /// The left-hand-side
    pub lhs: ReactionSide<E>,


    /// The right-hand-side
    pub rhs: ReactionSide<E>,


    // If it's an equilibrium
    pub is_equilibrium: bool
}


#[derive(Debug, Eq, PartialEq, Clone, Hash)]
/// A side of a reaction
pub struct ReactionSide<E: Element> {
    /// The compounds of this side
    pub compounds: Vec< ReactionCompound<E> >
}


#[derive(Debug, Eq, Clone, Hash)]
/// A reaction compound
pub struct ReactionCompound<E: Element> {
    /// The element it uses
    pub element: E,


    /// The amount of moles needed
    pub amount: u16
}



impl<E: Element> ElemReaction<E> {
    /// Convert a string representation of a reaction into one
    pub fn from_string(string: String) -> Option< ElemReaction<Ion> > {
        let mut token = String::new();

        let mut lhs = None;
        let mut rhs = None;
        let mut is_equilibrium = false;

        for c in string.chars() {
            if c == '<' || c == '>' || c == '⇌' || c == '→' {
                if lhs == None {
                    lhs = ReactionSide::<Ion>::from_string(token.clone());
                    token = String::new();
                }

                if c == '<' || c == '⇌' {
                    is_equilibrium = true;
                }

                continue;
            }

            token.push(c);
        }

        if token.len() > 0 {
            rhs = ReactionSide::<Ion>::from_string(token.clone());
        }


        if let (Some(lhs), Some(rhs)) = (lhs, rhs) {
            Some(ElemReaction {
                lhs: lhs,
                rhs: rhs,
                is_equilibrium: is_equilibrium
            })
        } else {
            None
        }
    }


    /// Get the sign of the equation ( → or ⇌ ), depending whether it is an equilibrium or not
    pub fn reaction_sign(&self) -> &str {
        if self.is_equilibrium {
            " ⇌ "
        } else {
            " → "
        }
    }


    /// Swap the equation
    pub fn swap(mut self) -> Self {
        let x = self.lhs;
        self.lhs = self.rhs;
        self.rhs = x;
        self
    }
}


impl<E: Element> ReactionSide<E> {
    /// Convert a string representation of a reactionside into one
    pub fn from_string(symbol: String) -> Option< ReactionSide<Ion> > {
        let mut compounds = vec! {};

        let mut token = String::new();
        for c in symbol.chars() {
            if is_whitespace!(c) {
                continue;
            }

            if c == '+' {
                if let Some(compound) = ReactionCompound::<Ion>::from_string(token) {
                    compounds.push(compound);
                }
                token = String::new();
                continue;
            }

            token.push(c);
        }

        if token.len() > 0 {
            if let Some(compound) = ReactionCompound::<Ion>::from_string(token) {
                compounds.push(compound);
            }
        }


        if compounds.len() > 0 {
            Some(ReactionSide {
                compounds: compounds
            })
        } else {
            None
        }
    }


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
        // FIXME: Calculate actual energy
        1_000.0 - (self.compounds.len() as f64) * 100.0
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


impl<E: Element> ReactionCompound<E> {
    /// Convert a string representation of a reaction compound into one
    pub fn from_string(symbol: String) -> Option< ReactionCompound<Ion> > {
        let mut amount: u16 = 0;
        let mut element = None;

        let mut set_amount = true;
        let mut token = String::new();

        for c in symbol.chars() {
            if set_amount && is_number!(c) {
                amount *= 10;
                amount += to_number!(c) as u16;
                continue;
            } else {
                set_amount = false;
            }

            token.push(c);
        }

        if token.len() > 0 {
            element = Ion::from_string(token);
        }

        if amount == 0 {
            amount = 1;
        }

        if let Some(element) = element {
            Some(ReactionCompound {
                amount: amount,
                element: element
            })
        } else {
            None
        }
    }
}


impl<E: Element> Reaction<E> for ElemReaction<E> {
    /// NOTE: This function is still a WIP!
    fn equalise(&self) -> bool {
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
                let difference = {
                    if l_amount > r_amount {
                        l_amount - r_amount
                    } else {
                        r_amount - l_amount
                    }
                };

                if difference > 0 {
                    // Increase right side
                    println!("We know what to do, but it's just not implemented yet.");
                } else {
                    // Increase left side
                    println!("We know what to do, but it's just not implemented yet.");
                }
            }
        }

        // Actually false
        return true;
    }


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


impl<E: Element> Add for ReactionSide<E> {
    type Output = ReactionSide<E>;

    /// Adding two ReactionSide's adds their compounds
    fn add(self, mut rhs: ReactionSide<E>) -> ReactionSide<E> {
        let mut compounds = self.compounds.clone();
        compounds.append(&mut rhs.compounds);

        ReactionSide {
            compounds: compounds
        }
    }
}


impl<E: Element> Mul<u16> for ReactionSide<E> {
    type Output = ReactionSide<E>;

    /// Multiplying a ReactionSide with a number
    /// multiplies the amount of all compounds of that side
    fn mul(self, rhs: u16) -> ReactionSide<E> {
        let mut compounds = self.compounds.clone();

        for mut compound in compounds.iter_mut() {
            compound.amount *= rhs;
        }

        ReactionSide {
            compounds: compounds
        }
    }
}


impl<E: Element> PartialEq for ReactionCompound<E> {
    /// Two ReactionCompound's are equal if their elements are equal
    fn eq(&self, rhs: &ReactionCompound<E>) -> bool {
        self.element == rhs.element
    }
}


impl<E: Element> Properties for ElemReaction<E>  {
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
        // Law of Conservation of Mass
        0.0
    }
}


impl<E: Element> Properties for ReactionSide<E>  {
    fn symbol(&self) -> String {
        let mut symbol = String::new();

        let mut first = true;
        for reaction_compound in self.compounds.iter() {
            if ! first {
                symbol += " + ";
            }
            first = false;

            symbol += &reaction_compound.symbol();
        }

        return symbol;
    }


    fn name(&self) -> String {
        let mut name = String::new();

        let mut first = true;
        for reaction_compound in self.compounds.iter() {
            if ! first {
                name += " + ";
            }
            first = false;

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


impl<E: Element> Properties for ReactionCompound<E>  {
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
            name += " ";
        }

        name += &self.element.name();

        return name;
    }


    fn mass(&self) -> AtomMass {
        return (self.amount as AtomMass) * self.element.mass();
    }
}


impl<E: Element> Element for ReactionCompound<E> {
    fn get_charge(&self) -> Option<IonCharge> {
        self.element.get_charge()
    }


    fn get_molecule(&self) -> Option<&Molecule> {
        self.element.get_molecule()
    }
}
