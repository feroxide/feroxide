use ion::Ion;


lazy_static! {
    pub static ref HYDROXIDE: Ion = Ion::from_string("OH;-".to_owned()).unwrap();
    pub static ref AMMONIUM: Ion = Ion::from_string("NH4;1+".to_owned()).unwrap();
    pub static ref SULPHATE: Ion = Ion::from_string("SO4;2-".to_owned()).unwrap();
}
