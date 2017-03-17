use atom::*;
use namings::*;
use trait_element::*;
use trait_properties::*;
use types::*;


#[derive(Debug, Eq, PartialEq, Clone)]
// And I'd love to include Copy, but Vec doesn't want me to
pub struct Molecule {
    pub compounds: Vec<MoleculeCompound>
}


#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct MoleculeCompound {
    pub atom: Atom,
    pub amount: u8
}



impl MoleculeCompound {
    pub fn from_atom(atom: Atom) -> MoleculeCompound {
        let amount = if atom.diatomic { 2 } else { 1 };

        MoleculeCompound { atom: atom, amount: amount }
    }
}


impl Properties for Molecule {
    /// Convert Molecule to symbol (2NaCl, Cl₂)
    fn symbol(&self) -> String {
        let mut symbol = String::new();

        for compound in self.compounds.iter() {
            symbol += &compound.symbol();
        }

        return symbol;
    }

    /// Convert Molecule to name (sodiumchlorine)
    fn name(&self) -> String {
        let mut name = String::new();

        for compound in self.compounds.iter() {
            name += &compound.name();
        }

        return name;
    }

    /// Calculate mass of Molecule
    fn mass(&self) -> AtomMass {
        let mut mass = 0.0;

        for compound in self.compounds.iter() {
            mass += compound.mass();
        }

        return mass;
    }
}


impl Properties for MoleculeCompound {
    fn symbol(&self) -> String {
        let mut symbol = String::new();

        symbol += &self.atom.symbol();

        if self.amount > 1 {
            symbol += &subscript(self.amount);
        }

        return symbol;
    }

    fn name(&self) -> String {
        let mut name = String::new();

        if self.amount > 1 {
            name += &number_to_greek(self.amount);
        }

        name += &self.atom.name();

        return name;
    }

    fn mass(&self) -> AtomMass {
        return (self.amount as AtomMass) * self.atom.mass;
    }
}


impl Element for Molecule {
    fn get_charge(&self) -> Option<IonCharge> {
        Some(0)
    }

    fn get_molecule(&self) -> Option<&Molecule> {
        Some(self)
    }
}
