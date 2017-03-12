use properties::*;

use types::*;

#[derive(Debug)]
pub struct Atom {
    pub number: AtomNumber,
    pub mass: AtomMass,
    pub symbol: &'static str,
    pub name: &'static str,
    pub group: AtomGroup,
    pub is_diatomic: bool
}

impl Properties for Atom {
    fn symbol(&self) -> String {
        self.symbol.to_owned()
    }

    fn name(&self) -> String {
        self.name.to_owned()
    }

    fn mass(&self) -> AtomMass {
        self.mass
    }
}
