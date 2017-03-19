use data_atoms::*;
use trait_properties::*;
use types::*;

use std::hash::*;


#[derive(Debug, Clone)]
/// An atom
pub struct Atom {
    /// Atom Number (Hydrogen: 1)
    pub number: AtomNumber,


    /// Atom Group (Hydrogen: 1)
    pub group: AtomGroup,


    /// Atom symbol (Hydrogen: H)
    pub symbol: &'static str,


    /// Atom name (Hydrogen: hydrogen)
    pub name: &'static str,


    /// Atom mass (Hydrogen: 1.008)
    pub mass: AtomMass,


    // Diatomic? (Hydrogen: true)
    pub diatomic: bool
}


impl Atom {
    /// Convert a string representation to an Atom
    pub fn from_string(symbol: String) -> Option<Atom> {
        for atom in ALL_ATOMS {
            if atom.symbol == symbol {
                return Some(atom.clone());
            }
        }

        None
    }
}


impl Eq for Atom {

}


impl PartialEq for Atom {
    /// Two atoms are equal when their atom numbers are equal
    fn eq(&self, rhs: &Atom) -> bool {
        self.number == rhs.number
    }
}


impl Hash for Atom {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Only the AtomNumber determines the hash
        self.number.hash(state);
    }
}


impl Properties for Atom {
    fn symbol(&self) -> String {
        self.symbol.to_owned()
    }


    fn name(&self) -> String {
        // NOTE: This case is temporary
        if self.name == "oxygen" {
            // Special suffix
            return "oxide".to_owned();
        }

        self.name.to_owned()
    }


    fn mass(&self) -> AtomMass {
        self.mass
    }
}
