use atom::*;
use namings::*;
use molecule::*;
use trait_element::*;
use trait_properties::*;
use types::*;

use std::collections::HashMap;


#[derive(Debug, Hash, Eq, PartialEq)]
pub enum IonData {
    CHARGE
}

pub type IonDataMap = HashMap<IonData, IonCharge>;


#[derive(Debug, Eq, PartialEq)]
pub struct Ion<'lifetime> {
    pub molecule: Molecule<'lifetime>,
    pub data: Option<IonDataMap>
}


/// Get the charge an atom has based on its group
pub fn charge_of_atom(atom: &Atom) -> IonCharge {
    let group = atom.group;
    let number = atom.number;

    if group == 0 { -1 }  // electron
    else if group == 1 { 1 }
    else if group == 2 { 2 }
    else if group == 15 && number <= 15 { -3 }
    else if group == 16 && number <= 34 { -2 }
    else if group == 17 && number <= 53 { -1 }
    else if group == 18 { 0 }

    else {
        panic!("No charge for atom {} known.", atom);
    }
}


impl<'lifetime> Ion<'lifetime> {
    /// Convert a Molecule into an Ion
    pub fn from_molecule(molecule: Molecule<'lifetime>) -> Ion<'lifetime> {
        Ion {
            molecule: molecule,
            data: Some(IonDataMap::charge(0))
        }
    }


    /// Calculate the charge of this Ion
    pub fn calculate_charge(&self) -> IonCharge {
        let mut charge = 0;

        for molecule_compound in self.molecule.compounds {
            let mol_charge =
                (molecule_compound.amount as IonCharge)
                * charge_of_atom(& molecule_compound.atom);

            charge += mol_charge;
        }

        // HACK: This seems to be correct for now
        charge = charge % 8;

        return charge;
    }
}


pub trait IonDataMapCharge {
    fn charge(_: IonCharge) -> IonDataMap;
    fn get_charge(&self) -> Option<&IonCharge>;
}

impl IonDataMapCharge for IonDataMap {
    /// Create an IonDataMap with given charge
    fn charge(charge: IonCharge) -> IonDataMap {
        let mut map = IonDataMap::new();
        map.insert(IonData::CHARGE, charge);

        return map;
    }


    /// Get the charge from an IonDataMap
    fn get_charge(&self) -> Option<&IonCharge> {
        self.get(&IonData::CHARGE)
    }
}


impl<'lifetime> Element for Ion<'lifetime> {
    fn get_charge(&self) -> Option<IonCharge> {
        if let Some(ref data) = self.data {
            if let Some(&charge) = data.get_charge() {
                Some(charge)
            } else {
                None
            }
        } else {
            Some(self.calculate_charge())
        }
    }


    fn get_molecule(&self) -> Option<&Molecule> {
        Some(&self.molecule)
    }
}


impl<'lifetime> Properties for Ion<'lifetime> {
    fn symbol(&self) -> String {
        let mut symbol = String::new();

        symbol += &self.molecule.symbol();

        if let Some(charge) = self.get_charge() {
            if charge != 0 {
                symbol += &ion_superscript(charge);
            }
        }

        return symbol;
    }


    fn name(&self) -> String {
        let mut name = String::new();

        name += &self.molecule.name();

        if let Some(charge) = self.get_charge() {
            if charge != 0 {
                name += "(";
                name += &number_to_roman(charge);
                name += ")";
            }
        }

        return name;
    }


    fn mass(&self) -> AtomMass {
        self.molecule.mass()
    }
}
