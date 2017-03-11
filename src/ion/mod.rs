use molecule::*;
use namings::*;

pub struct Ion<'lifetime> {
    pub compounds: &'lifetime [IonCompound<'lifetime>],
    pub total_charge: i8
}

pub struct IonCompound<'lifetime> {
    pub molecule: Molecule<'lifetime>,
    pub amount: u8,
    pub charge: i8
}

impl<'lifetime> Ion<'lifetime> {
    pub fn symbol(&self) -> String {
        let mut string = String::new();

        for compound in self.compounds {
            string += &compound.molecule.symbol();
        }

        string += &ion_superscript(self.total_charge);

        return string;
    }
}
