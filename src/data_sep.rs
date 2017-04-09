use ion::Ion;
use reaction::ElemReaction;
use trait_element::Element;
use trait_reaction::Reaction;
use types::*;

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};


// Reference: https://en.wikipedia.org/wiki/Standard_electrode_potential_(data_page)
// At doubt: Reference: Binas 6th edition, table 49


/// Get the hash of a reaction
pub fn reaction_to_hash<E: Element>(reaction: &ElemReaction<E>) -> u64 {
    let mut s = DefaultHasher::new();
    reaction.hash(&mut s);
    s.finish()
}


/// Get the Standerd Electrode Potential (SEP) of a reaction
pub fn get_sep<E: Element>(elem_reaction: &ElemReaction<E>) -> Option<SEP> {
    if let Some(sep) = SEPMAP.get(&reaction_to_hash(elem_reaction)) {
        Some(*sep)
    } else if let Some(sep) = SEPMAP.get(&reaction_to_hash(&elem_reaction.clone().swap())) {
        Some(*sep)
    } else {
        None
    }
}


// Macro to make it easier to add reactions to the SEPMAP
macro_rules! react_str_hash {
    ($s:expr) => {
        reaction_to_hash(
            & valid_or_panic(
                safe_unwrap_reaction(
                    ElemReaction::<Ion>::ion_from_string(
                        $s.to_owned()
                    ),
                    $s
                )
            )
        )
    }
}


/// Make sure the reaction is valid, panic otherwise
fn valid_or_panic<E: Element>(reaction: ElemReaction<E>) -> ElemReaction<E> {
    if !reaction.is_valid() {
        panic!("Invalid reaction: {}", reaction);
    }

    reaction
}


/// Check if the reaction is defined, then unwrap. Otherwise: panic!
fn safe_unwrap_reaction<E: Element>(reaction: Option<ElemReaction<E>>, s: &str) -> ElemReaction<E> {

    if reaction == None {
        panic!("Reaction failed to create: {}", s);
    }

    reaction.unwrap()
}


lazy_static! {
    pub static ref SEPMAP: HashMap<u64, SEP> = {
        let mut map = HashMap::new();

        // NOTE: These are the common ones, I think

        map.insert(react_str_hash!("Ca;1 + e <> Ca"), -3.8);
        map.insert(react_str_hash!("Li;1 + e <> Li"), -3.0401);
        map.insert(react_str_hash!("Cs;1 + e <> Cs"), -3.026);
        map.insert(react_str_hash!("Rb;1 + e <> Rb"), -2.98);
        map.insert(react_str_hash!("K;1 + e <> K"), -2.931);
        map.insert(react_str_hash!("Ba;2 + 2e <> Ba"), -2.912);
        map.insert(react_str_hash!("Fr;1 + e <> Fr"), -2.9);
        map.insert(react_str_hash!("Ca;2 + 2e <> Ca"), -2.899);
        map.insert(react_str_hash!("Na;1 + e <> Na"), -2.71);
        map.insert(react_str_hash!("Mg;1 + e <> Mg"), -2.70);
        map.insert(react_str_hash!("Mg;2 + 2e <> Mg"), -2.372);
        map.insert(react_str_hash!("H2 + 2e <> 2H;-"), -2.23);
        map.insert(react_str_hash!("Be;2 + 2e <> Be"), -1.847);
        map.insert(react_str_hash!("Al;3 + 3e <> Al"), -1.662);
        map.insert(react_str_hash!("Mn;2 + 2e <> Mn"), -1.185);
        map.insert(react_str_hash!("Sn + 4H;1 + 4e <> SnH4"), -1.07);
        map.insert(react_str_hash!("2H2O + 2e <> H2 + 2OH;-"), -0.8277);
        map.insert(react_str_hash!("Zn;2 + 2e <> Zn"), -0.7618);
        map.insert(react_str_hash!("Fe;2 + 2e <> Fe"), -0.44);
        map.insert(react_str_hash!("Cr;3 + e <> Cr;2"), -0.42);
        map.insert(react_str_hash!("Sn;2 + 2e <> Sn"), -0.13);
        map.insert(react_str_hash!("Fe;3 + 3e <> Fe"), -0.04);

        map.insert(react_str_hash!("2H;1 + 2e <> H2"), 0.0000);

        map.insert(react_str_hash!("S4O6;2- + 2e <> 2S2O3;2-"), 0.08);
        map.insert(react_str_hash!("S + 2H;1 + 2e <> H2S"), 0.14);
        map.insert(react_str_hash!("Sn;4 + 2e <> Sn;2"), 0.15);
        map.insert(react_str_hash!("Cu;2 + e <> Cu;1"), 0.159);
        map.insert(react_str_hash!("HSO4;- + 3H;1 + 2e <> SO2 + 2H2O"), 0.16);
        map.insert(react_str_hash!("SO4;- + 4H;1 + 2e <> SO2 + 2H2O"), 0.17);
        map.insert(react_str_hash!("Cu;2 + 2e <> Cu"), 0.337);
        map.insert(react_str_hash!("SO2 + 4H;1 + 4e <> S + 2H2O"), 0.50);
        map.insert(react_str_hash!("Cu;1 + e <> Cu"), 0.520);
        map.insert(react_str_hash!("I3;- + 2e <> 3I;-"), 0.53);
        map.insert(react_str_hash!("I2 + 2e <> 2I;-"), 0.54);
        map.insert(react_str_hash!("S2O3;2- + 6H;1 + 4e <> 2S + 3H2O"), 0.60);
        map.insert(react_str_hash!("O2 + 2H;1 + 2e <> H2O2"), 0.70);
        map.insert(react_str_hash!("Fe;3 + e <> Fe;2"), 0.77);
        map.insert(react_str_hash!("Ag;1 + e <> Ag"), 0.7996);
        map.insert(react_str_hash!("NO3;- + 2H;1 + e <> NO2 + H2O"), 0.80);
        map.insert(react_str_hash!("MnO4;- + H;1 + e <> HMnO4;-"), 0.90);
        map.insert(react_str_hash!("MnO2 + 4H;1 + e <> Mn;3 + 2H2O"), 0.95);
        map.insert(react_str_hash!("NO3;- + 4H;1 + 3e <> NO + 2H2O"), 0.958);
        map.insert(react_str_hash!("O2 + 4H;1 + 4e <> 2H2O"), 1.229);
        map.insert(react_str_hash!("MnO2 + 4H;1 + 2e <> Mn;2 + 2H2O"), 1.23);
        map.insert(react_str_hash!("Cr2O7;2- + 14H;1 + 6e <> 2Cr;3 + 7H2O"), 1.33);
        map.insert(react_str_hash!("Cl2 + 2e <> 2Cl;-"), 1.36);
        map.insert(react_str_hash!("MnO4;- + 8H;1 + 5e <> Mn;2 + 4H2O"), 1.51);
        map.insert(react_str_hash!("Au;3 + 3e <> Au"), 1.52);
        map.insert(react_str_hash!("Pb;4 + 2e <> Pb;2"), 1.69);
        map.insert(react_str_hash!("MnO4;- + 4H;1 + 3e <> MnO2 + 2H2O"), 1.70);
        map.insert(react_str_hash!("H2O2 + 2H;1 + 2e <> 2H2O"), 1.78);
        map.insert(react_str_hash!("Co;3 + e <> Co;2"), 1.82);
        map.insert(react_str_hash!("Au;1 + e <> Au"), 1.83);
        map.insert(react_str_hash!("Ag;2 + e <> Ag;1"), 1.98);
        map.insert(react_str_hash!("S2O8;2- + 2e <> 2SO4;2-"), 2.010);
        map.insert(react_str_hash!("HMnO4;- + 3H;1 + 2e <> MnO2 + 2H2O"), 2.09);
        map.insert(react_str_hash!("FeO4;2- + 3e + 8H;1 <> Fe;3 + 4H2O"), 2.20);
        map.insert(react_str_hash!("F2 + 2e <> 2F;-"), 2.87);
        map.insert(react_str_hash!("F2 + 2H;1 + 2e <> 2HF"), 3.05);
        map.insert(react_str_hash!("KrF2 + 2e <> Kr + 2F;-"), 3.27);


        // NOTE: The Wikipedia reference says something different,
        // yet according to the Binas this is the right one.
        // Since the value on the Wikipedia page (1.692) does not look "in-place",
        // I assumed the Binas one is correct here.
        map.insert(react_str_hash!("MnO4;- + 2H2O + 3e <> MnO2 + 4OH;-"), 0.60);



        map
    };
}
