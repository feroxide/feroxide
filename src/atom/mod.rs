pub type AtomNumber = u16;

#[derive(Debug)]
pub struct Atom {
    pub number: AtomNumber,
    pub mass: f32,
    pub symbol: &'static str,
    pub name: &'static str,
    pub is_diatomic: bool
}
