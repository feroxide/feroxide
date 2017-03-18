use trait_properties::*;
use types::*;

use std::hash::*;


#[derive(Debug, Clone)]
pub struct Atom {
    pub number: AtomNumber,
    pub group: AtomGroup,
    pub symbol: &'static str,
    pub name: &'static str,
    pub mass: AtomMass,
    pub diatomic: bool
}



impl Eq for Atom {}
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
