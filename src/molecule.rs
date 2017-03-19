use atom::*;
use namings::*;
use trait_element::*;
use trait_properties::*;
use types::*;


#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Molecule {
    pub compounds: Vec<MoleculeCompound>
}


#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct MoleculeCompound {
    pub atom: Atom,
    pub amount: u8
}


macro_rules! is_upper {
    ($c: expr) => {
        $c >= 'A' && $c <= 'Z'
    }
}

macro_rules! is_lower {
    ($c: expr) => {
        $c >= 'a' && $c <= 'z'
    }
}

macro_rules! is_number {
    ($c: expr) => {
        $c >= '0' && $c <= '9'
    }
}

macro_rules! is_letter {
    ($c: expr) => {
        is_upper!($c) || is_lower!($c)
    }
}

macro_rules! to_number {
    ($c: expr) => {
        ($c as u8) - ('0' as u8)
    }
}


impl Molecule {
    pub fn from_string(string: String) -> Molecule {
        let mut compounds = vec!{};

        let mut token = String::new();

        for c in string.chars() {
            // Ignore whitespace
            if c == ' ' {
                continue;
            }

            if is_upper!(c) && token.len() > 0 {
                let compound = MoleculeCompound::from_string(token).unwrap();

                compounds.push(compound);
                token = String::new();
            }

            token.push(c);
        }

        // If some tokens remain
        if token.len() > 0 {
            let compound = MoleculeCompound::from_string(token).unwrap();
            compounds.push(compound);
        }

        Molecule {
            compounds: compounds
        }
    }
}



impl MoleculeCompound {
    /// Takes a symbol string representing a MoleculeCompound, and turns it into one
    pub fn from_string(string: String) -> Option<MoleculeCompound> {
        let mut atom_symbol = String::new();
        let mut amount = 0;

        for c in string.chars() {
            if is_letter!(c) {
                atom_symbol.push(c);
            } else {
                amount *= 10;
                amount += to_number!(c);
            }
        }

        // If no amount given, assume 1
        if amount == 0 {
            amount = 1;
        }


        if let Some(atom) = Atom::from_string(atom_symbol) {
            Some(MoleculeCompound {
                atom: atom,
                amount: amount
            })
        } else {
            None
        }
    }


    /// Converts an Atom into a MoleculeCompound, paying attention to diatomic ones
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
