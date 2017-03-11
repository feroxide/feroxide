pub type AtomNumber = u16;

#[derive(Debug)]
pub struct Atom {
    pub number: AtomNumber,
    pub mass: f32,
    pub symbol: &'static str,
    pub name: &'static str,
    pub is_diatomic: bool
}


impl Atom {
    pub fn data(&self) -> String {
        return format!("{}({}), {}, {} u, {}", self.symbol, self.name, self.number, self.mass, self.is_diatomic);
    }
}
