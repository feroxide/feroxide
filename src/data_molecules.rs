use molecule::Molecule;


lazy_static! {
    pub static ref WATER: Molecule = Molecule::from_string("H2O").unwrap();
    pub static ref CO2: Molecule = Molecule::from_string("CO2").unwrap();
    pub static ref AMMONIA: Molecule = Molecule::from_string("NH3").unwrap();
    pub static ref SUGAR: Molecule = Molecule::from_string("C12H22O11").unwrap();
}
