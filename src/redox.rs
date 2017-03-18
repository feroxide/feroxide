use data_sep::*;
use math::*;
use reaction::*;
use trait_element::*;
use trait_properties::*;
use trait_reaction::*;
use types::*;


#[derive(Debug, Eq, PartialEq, Clone)]
pub struct RedoxReaction<E: Element> {
    pub reductor: ElemReaction<E>,
    pub oxidator: ElemReaction<E>
}


impl<E> Reaction<E> for RedoxReaction<E> where E: Element {
    fn equalise(&self) -> bool {
        // Assume reaction is correct for now
        true
    }


    fn is_valid(&self) -> bool {
        // oxidator > reductor
        get_sep(&self.oxidator) > get_sep(&self.reductor) && self.elem_reaction().is_valid()
    }


    fn energy_cost(&self) -> Energy {
        self.reductor.energy_cost() + self.oxidator.energy_cost()
    }


    fn elem_reaction(&self) -> ElemReaction<E> {
        // Assuming .rhs and .lhs are equalised

        let red_charge;
        let oxi_charge;

        // ehm... let me explain these two

        // Get reductor charge by searching for the electron, then getting that amount
        if let Some(red_elec_pos) = self.reductor.rhs.compounds.iter().position(|x|
            x.element.get_molecule().unwrap()
            .compounds.get(0).unwrap()
            .atom.number == 0
        ) {
            red_charge = self.reductor.rhs.compounds.get(red_elec_pos).unwrap().amount;
        } else if let Some(red_elec_pos) = self.reductor.lhs.compounds.iter().position(|x|
            x.element.get_molecule().unwrap()
            .compounds.get(0).unwrap()
            .atom.number == 0
        ) {
            red_charge = self.reductor.lhs.compounds.get(red_elec_pos).unwrap().amount;
        } else {
            panic!("Reductor has no electrons!");
        }


        // Get oxidator charge by searching for the electron, then getting that amount
        if let Some(oxi_elec_pos) = self.oxidator.lhs.compounds.iter().position(|x|
            x.element.get_molecule().unwrap()
            .compounds.get(0).unwrap()
            .atom.number == 0
        ) {
            oxi_charge = self.oxidator.lhs.compounds.get(oxi_elec_pos).unwrap().amount;
        } else if let Some(oxi_elec_pos) = self.oxidator.rhs.compounds.iter().position(|x|
            x.element.get_molecule().unwrap()
            .compounds.get(0).unwrap()
            .atom.number == 0
        ) {
            oxi_charge = self.oxidator.rhs.compounds.get(oxi_elec_pos).unwrap().amount;
        } else {
            panic!("Oxidator has no electrons!");
        }


        // Make sure that 4/2 or 2/4 gets converted to 2/1 or 1/2 first
        let gcd = gcd(red_charge as i32, oxi_charge as i32) as u16;
        let red_mult = oxi_charge / gcd;
        let oxi_mult = red_charge / gcd;


        let x = ElemReaction {
            lhs: self.reductor.lhs.clone() * red_mult + self.oxidator.lhs.clone() * oxi_mult,
            rhs: self.reductor.rhs.clone() * red_mult + self.oxidator.rhs.clone() * oxi_mult,

            is_equilibrium: true
        };

        x
    }
}


impl<E: Element> Properties for RedoxReaction<E> {
    fn symbol(&self) -> String {
        let mut symbol = String::new();

        symbol += "oxidator: ";
        symbol += &self.oxidator.symbol();
        symbol += "\n";
        symbol += "reductor: ";
        symbol += &self.reductor.symbol();

        return symbol;
    }


    fn name(&self) -> String {
        let mut name = String::new();

        name += "oxidator: ";
        name += &self.oxidator.name();
        name += "\n";
        name += "reductor: ";
        name += &self.reductor.name();

        return name;
    }


    fn mass(&self) -> AtomMass {
        // Law of Conservation of Mass
        0.0
    }
}
