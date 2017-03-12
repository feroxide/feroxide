use element::*;
use types::*;

pub struct Container<'lifetime> {
    pub contents: &'lifetime [Element],
    pub available_energy: Energy
}
