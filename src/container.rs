use molecule::*;
use reaction::*;
use trait_element::*;
use trait_properties::*;
use trait_reaction::*;
use types::*;

use std::hash::*;


#[derive(Debug)]
pub struct Container<E> where E: Element {
    pub contents: Vec< ContainerCompound<E> >,
    pub available_energy: Energy
}


#[derive(Debug, Clone)]
pub struct ContainerCompound<E> where E: Element {
    pub element: E,
    pub moles: Moles
}


/// Convert a given ReactionCompound into a ContainerCompound
pub fn rc_to_cc<E>(rc: ReactionCompound<E>) -> ContainerCompound<E> where E: Element {
    ContainerCompound {
        element: rc.element,
        moles: rc.amount as Moles
    }
}


impl<E> Container<E> where E: Element {
    /// Applies given reaction to container
    /// Removing the elements on the left-hand side
    /// and adding the elements on the right-hand side.
    /// If there is enough energy for the reaction, that amount will be consumed
    /// otherwise the reaction won't occur.
    /// Returns if the reaction succeeded
    pub fn react<R>(&mut self, reaction: &R) -> bool where R: Reaction<E> {
        // Get required items
        let required_energy = reaction.energy_cost();
        let mut required_elements = vec! {};
        let mut resulting_elements = vec! {};

        // Convert lhs.compounds into ContainerCompound's
        for rc in reaction.elem_reaction().lhs.compounds.iter() {
            let cc = rc_to_cc(rc.clone());

            required_elements.push(cc);
        }

        // Convert rhs.compounds into ContainerCompound's
        for rc in reaction.elem_reaction().rhs.compounds.iter() {
            let cc = rc_to_cc(rc.clone());

            resulting_elements.push(cc);
        }

        // Check if the container has enough energy
        if self.available_energy < required_energy {
            println!("####    Not enough energy");
            return false;
        }

        // Check if the container has the required elements
        if ! self.has_elements(&required_elements) {
            println!("####    Not enough elements");
            return false;
        }

        // Subtract needed energy (or add, in case of an exothermic reaction)
        self.available_energy -= required_energy;

        // Remove required elements
        self.remove_elements(&required_elements);

        // Add reaction results
        self.add_elements(&resulting_elements);

        return true;
    }


    /// Check if the container has all given elements
    pub fn has_elements(&mut self, elements: &Vec< ContainerCompound<E> >) -> bool {
        'outer: for element in elements {
            // Find element in self.contents
            if let Some(position) = self.contents.iter().position(|comp| comp == element) {
                let compound = self.contents.get(position).unwrap();

                // Check if more elements are required than available
                if element.moles > compound.moles {
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
    pub fn remove_elements(&mut self, elements: &Vec< ContainerCompound<E> >) {
        for element in elements {
            // Find element in self.contents
            if let Some(position) = self.contents.iter().position(|comp| comp == element) {
                if {
                    let mut compound = self.contents.get_mut(position).unwrap();

                    // Check if we have enough
                    if compound.moles < element.moles {
                        panic!("Can't remove element {} (not enough)", element.symbol());
                    }

                    // Remove amount
                    compound.moles -= element.moles;

                    // If none is available anymore, remove element from container in its entirety
                    compound.moles == 0.0
                } {
                    self.contents.remove(position);
                }
            } else {
                println!("Can't remove element {} (not found)", element.symbol());
            }
        }
    }


    /// Add given elements to container
    pub fn add_elements(&mut self, elements: &Vec< ContainerCompound<E> >) {
        for element in elements.into_iter() {
            // Find element in self.contents
            if let Some(position) = self.contents.iter().position(|comp| comp == element) {
                let mut compound = self.contents.get_mut(position).unwrap();

                // Add amount
                compound.moles += element.moles;
            } else {
                // If the element is not found in the container, add it

                self.contents.push(element.clone());
            }
        }
    }


    /// Convert container to a nice string for displaying
    pub fn stringify(&self) -> String {
        let mut string = String::new();

        let mut first = true;
        for compound in self.contents.iter() {
            if compound.moles > 0.0 {
                if ! first {
                    string += " + ";
                }
                first = false;

                string += &compound.stringify();
            }
        }

        string += &" [";
        string += &self.available_energy.to_string();
        string += &" J]";

        return string;
    }
}


impl<E> Eq for ContainerCompound<E> where E: Element {}

impl<E> PartialEq for ContainerCompound<E> where E: Element {
    /// Two container compounds are equal when their elements are equal
    fn eq(&self, rhs: &ContainerCompound<E>) -> bool {
        self.element == rhs.element
    }
}


impl<E> Hash for ContainerCompound<E> where E: Element {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.element.hash(state);
    }
}


impl<E> Element for ContainerCompound<E> where E: Element {
    fn get_charge(&self) -> Option<IonCharge> {
        self.element.get_charge()
    }

    fn get_molecule(&self) -> Option<&Molecule> {
        self.element.get_molecule()
    }
}


impl<E> Properties for ContainerCompound<E> where E: Element {
    fn symbol(&self) -> String {
        let mut symbol = String::new();
        symbol += &self.moles.to_string();
        symbol += &self.element.symbol();
        return symbol;
    }

    fn name(&self) -> String {
        let mut name = String::new();
        name += &self.moles.to_string();
        name += &self.element.name();
        return name;
    }

    fn mass(&self) -> AtomMass {
        self.element.mass()
    }
}
