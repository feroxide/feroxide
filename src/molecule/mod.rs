use atom::Atom;
use namings;

pub struct MoleculeCompound<'lifetime> {
    pub atom: &'lifetime Atom,
    pub amount: u8
}

pub struct Molecule<'lifetime> {
    pub compounds: &'lifetime Vec< MoleculeCompound<'lifetime> >,
}


impl<'lifetime> MoleculeCompound<'lifetime> {
    pub fn symbol(&self) -> String {
        let mut symbol = String::new();

        symbol += self.atom.symbol;

        if self.amount > 1 {
            symbol += & namings::subscript(self.amount);
        }

        return symbol;
    }

    pub fn mass(&self) -> f32 {
        return (self.amount as f32) * self.atom.mass;
    }
}


impl<'lifetime> Molecule<'lifetime> {
    pub fn symbol(&self) -> String {
        let mut symbol = String::new();

        for compound in self.compounds {
            symbol += & compound.symbol();
        }

        return symbol;
    }

    pub fn name(&self) -> String {
        let mut name = String::new();

        for compound in self.compounds {
            if compound.amount > 1 {
                name += &namings::number_to_greek(compound.amount);
            }

            name += compound.atom.name;
        }

        return name;
    }

    pub fn mass(&self) -> f32 {
        let mut mass = 0.0;

        for compound in self.compounds {
            mass += compound.mass();
        }

        return mass;
    }
}
