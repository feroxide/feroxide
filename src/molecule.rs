use atom::*;
use namings::*;
use trait_element::*;
use trait_properties::*;
use types::*;


#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Molecule<'lifetime> {
    pub compounds: &'lifetime [MoleculeCompound]
}


#[derive(Debug, Eq, PartialEq)]
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


impl<'lifetime> Properties for Molecule<'lifetime> {
    /// Convert Molecule to symbol (2NaCl, Cl₂)
    fn symbol(&self) -> String {
        let mut symbol = String::new();

        for compound in self.compounds {
            symbol += &compound.symbol();
        }

        return symbol;
    }

    /// Convert Molecule to name (sodiumchlorine)
    fn name(&self) -> String {
        let mut name = String::new();

        for compound in self.compounds {
            name += &compound.name();
        }

        return name;
    }

    /// Calculate mass of Molecule
    fn mass(&self) -> AtomMass {
        let mut mass = 0.0;

        for compound in self.compounds {
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


impl<'lifetime> Element for Molecule<'lifetime> {
    fn get_charge(&self) -> Option<IonCharge> {
        Some(0)
    }

    fn get_molecule(&self) -> Option<&Molecule> {
        Some(self)
    }
}
