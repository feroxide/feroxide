use atom::*;

use element::*;
use properties::*;

use namings::*;
use types::*;

#[derive(Debug)]
pub struct Molecule<'lifetime> {
    pub compounds: &'lifetime [MoleculeCompound<'lifetime>]
}

#[derive(Debug)]
pub struct MoleculeCompound<'lifetime> {
    pub atom: &'lifetime Atom,
    pub amount: u8
}


impl<'lifetime> Element for Molecule<'lifetime> {
    fn get_molecule(&self) -> Option<&Molecule> {
        Some(self)
    }
}


impl<'lifetime> MoleculeCompound<'lifetime> {
    pub fn from_atom(atom: &'lifetime Atom) -> MoleculeCompound<'lifetime> {
        let amount = if atom.is_diatomic { 2 } else { 1 };

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

impl<'lifetime> Properties for MoleculeCompound<'lifetime> {
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
