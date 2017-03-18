use reaction::*;
use trait_element::*;
use trait_properties::*;
use types::*;


pub trait Reaction<E>: Properties where E: Element {
    /// Check if the reaction is valid by comparing the amount of total atoms on both sides,
    /// and by checking if the total charge on both sides is equal
    fn is_valid(&self) -> bool;

    /// Calculate the cost of this reaction
    /// This can be negative in case of an exothermic reaction
    fn energy_cost(&self) -> Energy;

    fn elem_reaction(&self) -> ElemReaction<E>;
}
