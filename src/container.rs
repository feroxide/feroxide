use reaction::*;
use trait_element::*;
use types::*;


#[derive(Debug)]
pub struct Container<T> where T: Element {
    pub contents: Vec< ReactionCompound<T> >,
    pub available_energy: Energy
}



impl<T> Container<T> where T: Element {
    /// Applies given reaction to container
    /// Removing the elements on the left-hand side
    /// and adding the elements on the right-hand side.
    /// If there is enough energy for the reaction, that amount will be consumed
    /// otherwise the reaction won't occur.
    pub fn react(&mut self, reaction: &Reaction<T>) {
        // Get required items
        let required_energy = reaction.energy_cost();
        let ref required_elements = reaction.lhs.compounds;
        let ref resulting_elements = reaction.rhs.compounds;

        // Check if the container has enough energy
        if self.available_energy < required_energy {
            return;
        }

        // Check if the container has the required elements
        if ! self.has_elements(required_elements) {
            return;
        }

        // Subtract needed energy (or add, in case of an exothermic reaction)
        self.available_energy -= required_energy;

        // Remove required elements
        self.remove_elements(required_elements);
        // Add reaction results
        self.add_elements(resulting_elements);
    }


    /// Check if the container has all given elements
    pub fn has_elements(&mut self, elements: &Vec< ReactionCompound<T> >) -> bool {
        'outer: for element in elements {
            // Find element in self.contents
            if let Some(position) = self.contents.iter_mut().position(|comp| comp == element) {
                let compound = self.contents.get(position).unwrap();

                // Check if more elements are required than available
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


    /// Remove given elements from container
    pub fn remove_elements(&mut self, elements: &Vec< ReactionCompound<T> >) {
        for element in elements {
            // Find element in self.contents
            if let Some(position) = self.contents.iter_mut().position(|comp| comp == element) {
                let mut compound = self.contents.get_mut(position).unwrap();

                // Check if we have enough
                if compound.amount < element.amount {
                    panic!("Can't remove element {} (not enough)", element.to_string());
                }

                // Remove amount
                compound.amount -= element.amount;

                // If none is available anymore, remove element from container in its entirety
                if compound.amount == 0 {
                    // FIXME:
                    // self.contents.remove(position);
                    println!("## Removing elements from containers is currently not supported.");
                }
            } else {
                println!("Can't remove element {} (not found)", element.to_string());
            }
        }
    }


    /// Add given elements to container
    pub fn add_elements(&mut self, elements: &Vec< ReactionCompound<T> >) {
        for element in elements {
            // Find element in selfcontents
            if let Some(position) = self.contents.iter_mut().position(|comp| comp == element) {
                let mut compound = self.contents.get_mut(position).unwrap();

                // Add amount
                compound.amount += element.amount;
            } else {
                // If the element is not found in the container, add it

                // FIXME:
                // self.contents.push(element);
                println!("## Adding elements to containers is currently not supported.");
            }
        }
    }


    /// Convert container to a nice string for displaying
    pub fn stringify(&self) -> String {
        let mut string = String::new();

        let mut first = true;
        for compound in self.contents.iter() {
            if compound.amount > 0 {
                if ! first {
                    string += " + ";
                }
                first = false;

                string += &compound.amount.to_string();
                string += &compound.stringify();
            }
        }

        string += &" [";
        string += &self.available_energy.to_string();
        string += &" J]";

        return string;
    }
}
