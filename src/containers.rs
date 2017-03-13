use reaction::*;

use element::*;
use types::*;


#[derive(Debug)]
pub struct Container<T> where T: Element {
    // pub contents: &'lifetime mut [&'lifetime mut ReactionCompound<'lifetime, T>],
    pub contents: Vec< ReactionCompound<T> >,
    pub available_energy: Energy
}



impl<T> Container<T> where T: Element {
    pub fn react(&mut self, reaction: Reaction<T>) {
        let required_energy = reaction.energy_cost();
        let required_elements = reaction.lhs.compounds;

        if self.available_energy < required_energy {
            return;
        }

        if ! self.has_elements(required_elements) {
            return;
        }

        self.available_energy -= required_energy;

        self.remove_elements(required_elements);
        self.add_elements(reaction.rhs.compounds);
    }

    pub fn has_elements(&mut self, elements: &[ReactionCompound<T>]) -> bool {
        'outer: for element in elements {
            // Find element in selfcontents
            if let Some(position) = self.contents.iter_mut().position(|comp| comp == element) {
                let compound = self.contents.get(position).unwrap();

                // More elements are required than available
                if element.amount > compound.amount {
                    return false;
                }

                continue 'outer;
            }

            // Element is not available
            return false;
        }

        return true;
    }

    pub fn remove_elements(&mut self, elements: &[ReactionCompound<T>]) {
        for element in elements {
            // Find element in selfcontents
            if let Some(position) = self.contents.iter_mut().position(|comp| comp == element) {
                let mut compound = self.contents.get_mut(position).unwrap();

                if compound.amount < element.amount {
                    panic!("Can't remove element");
                }

                compound.amount -= element.amount;

                if compound.amount == 0 {
                    // FIXME:
                    // self.contents.remove(position);
                    println!("####    Removing elements from containers is currently not yet supported.");
                }

                return;
            }
        }
    }

    pub fn add_elements(&mut self, elements: &[ReactionCompound<T>]) {
        for element in elements {
            // Find element in selfcontents
            if let Some(position) = self.contents.iter_mut().position(|comp| comp == element) {
                let mut compound = self.contents.get_mut(position).unwrap();

                compound.amount += element.amount;
            } else {
                // FIXME:
                // self.contents.push(*element);
                println!("####    Adding elements to containers is currently not yet supported.");
            }
        }
    }
}
