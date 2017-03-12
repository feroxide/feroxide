use reaction::*;

use element::*;
use types::*;

#[derive(Debug)]
pub struct Container<'lifetime, T: 'lifetime> where T: Element {
    pub contents: &'lifetime [&'lifetime ReactionCompound<'lifetime, T>],
    pub available_energy: Energy
}



impl<'lifetime, T> Container<'lifetime, T> where T: Element {
    pub fn react(&mut self, reaction: &'lifetime Reaction<T>) {
        let required_energy = reaction.energy_cost();
        let required_elements = reaction.lhs.compounds;

        if self.available_energy < required_energy {
            return;
        }

        if ! self.has_elements(required_elements) {
            return;
        }

        self.remove_energy(required_energy);
        self.remove_elements(required_elements);
        self.add_elements(reaction.rhs.compounds);
    }

    pub fn remove_energy(&mut self, required_energy: Energy) {
        self.available_energy -= required_energy;
    }

    pub fn has_elements(&self, elements: &'lifetime [&'lifetime ReactionCompound<T>]) -> bool {
        'outer: for el in elements {
            for compound in self.contents {
                if el == compound {

                    // More elements are required than available
                    if el.amount > compound.amount {
                        return false;
                    }

                    continue 'outer;
                }
            }

            // Element is not available
            return false;
        }

        return true;
    }

    pub fn remove_elements(&mut self, elements: &'lifetime [&'lifetime ReactionCompound<T>]) {
        'outer: for el in elements {
            for compound in self.contents {
                if el == compound {

                    if el.amount > compound.amount {
                        panic!("Can't remove element");
                    }

                    // compound.amount -= el.amount;

                    return;
                }
            }
        }
    }

    pub fn add_elements(&mut self, _elements: &[&'lifetime ReactionCompound<T>]) {

    }
}
