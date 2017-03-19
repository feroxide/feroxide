use atom::*;
use electron::*;
use namings::*;
use molecule::*;
use trait_element::*;
use trait_properties::*;
use types::*;


#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Ion {
    pub molecule: Molecule,
    pub charge: Option<IonCharge>
}


/// Get the charge an atom has based on its group
pub fn charge_of_atom(atom: Atom) -> Option<IonCharge> {
    let group = atom.group;
    let number = atom.number;

    if group == 1 { Some(1) }
    else if group == 2 { Some(2) }
    else if group == 15 && number <= 15 { Some(-3) }
    else if group == 16 && number <= 34 { Some(-2) }
    else if group == 17 && number <= 53 { Some(-1) }
    else if group == 18 { Some(0) }

    else {
        None
    }
}


impl Ion {
    pub fn from_string(symbol: String) -> Option< Ion > {

        println!("Ion from string \"{}\"", symbol);

        let mut molecule = None;
        let mut charge: IonCharge = 0;
        let mut is_negative = false;

        let mut token = String::new();
        let mut set_charge = false;
        for c in symbol.chars() {
            if c == ';' {
                // Electron
                if token == "e" {
                    return Some(ELECTRON.clone());
                }

                molecule = Molecule::from_string(token);
                token = String::new();
                set_charge = true;
                continue;
            }

            if set_charge {
                if c == '-' {
                    is_negative = true;
                    continue;
                }

                charge *= 10;
                charge += to_number!(c) as i8;
                continue;
            }

            token.push(c);
        }

        // It's just a molecule
        if token.len() > 0 && ! set_charge {
            // Electron
            if token == "e" {
                return Some(ELECTRON.clone());
            }

            molecule = Molecule::from_string(token);
        }

        if is_negative {
            // assume - to mean -1
            if charge == 0 {
                charge = -1;
            } else {
                charge *= -1;
            }
        }

        if let Some(molecule) = molecule {
            Some(Ion {
                molecule: molecule,
                charge: Some(charge)
            })
        } else {
            None
        }
    }


    /// Convert a Molecule into an Ion
    pub fn from_molecule(molecule: Molecule) -> Ion {
        Ion {
            molecule: molecule,
            charge: None // Will be calculated
        }
    }


    /// Calculate the charge of this Ion
    pub fn calculate_charge(&self) -> Option<IonCharge> {
        let mut charge = 0;

        for molecule_compound in self.molecule.compounds.iter() {
            if let Some(atom_charge) = charge_of_atom(molecule_compound.atom.clone()) {
                let mol_charge = (molecule_compound.amount as IonCharge) * atom_charge;

                charge += mol_charge;
            }
        }

        // HACK: This seems to be correct for now
        charge = charge % 8;

        Some(charge)
    }
}


impl Element for Ion {
    fn get_charge(&self) -> Option<IonCharge> {
        if let Some(charge) = self.charge {
            Some(charge)
        } else {
            self.calculate_charge()
        }
    }


    fn get_molecule(&self) -> Option<&Molecule> {
        Some(&self.molecule)
    }
}


impl Properties for Ion {
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
