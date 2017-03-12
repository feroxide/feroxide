pub type AtomNumber = u8;
pub type AtomMass = f32;

#[derive(Debug)]
pub struct Atom {
    pub number: AtomNumber,
    pub mass: AtomMass,
    pub symbol: &'static str,
    pub name: &'static str,
    pub group: u8,
    pub is_diatomic: bool
}


pub trait Properties {
    fn symbol(&self) -> String;
    fn name(&self) -> String;
    fn mass(&self) -> AtomMass;

    fn to_string(&self) -> String { self.symbol() }
}


impl Properties for Atom {
    fn symbol(&self) -> String {
        self.symbol.to_owned()
    }

    fn name(&self) -> String {
        self.name.to_owned()
    }

    fn mass(&self) -> AtomMass {
        self.mass
    }
}
