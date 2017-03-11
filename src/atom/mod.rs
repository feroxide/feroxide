pub struct Atom {
    pub atom_number: u16,
    pub mass: f32,
    pub symbol: &'static str,
    pub name: &'static str
}


impl Atom {
    pub fn data(&self) -> String {
        return format!("{}({}), {}, {} u", self.symbol, self.name, self.atom_number, self.mass);
    }
}
