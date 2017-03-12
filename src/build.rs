#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::io::prelude::*;
use std::fs::File;

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    pub atoms: HashMap<String, Atom>
}

#[derive(Serialize, Deserialize, Debug)]
struct Atom {
    pub number: u16,
    pub symbol: String,
    pub name: String,
    pub mass: f32,
    pub diatomic: bool
}


fn main() {
    let mut atoms_rs_file = File::create("src/atoms.rs").unwrap();
    let mut atoms_toml_file = File::open("src/atoms.toml").unwrap();

    /*
    let mut atoms: HashMap<String, Atom> = HashMap::new();

    atoms.insert("hydrogen".to_owned(), Atom { number: 1, symbol: "H".to_owned(), name: "H".to_owned(), mass: 1.0, diatomic: true });
    atoms.insert("helium".to_owned(), Atom { number: 2, symbol: "He".to_owned(), name: "He".to_owned(), mass: 3.0, diatomic: true });

    let config = Config {
        atom: atoms
    };

    let _ = atoms_rs_file.write_all(  toml::to_string(&config).unwrap().as_bytes()  );
    */

    let mut atoms_toml = String::new();
    let _ = atoms_toml_file.read_to_string(&mut atoms_toml);

    let config: Config = toml::from_str(&atoms_toml).unwrap();

    let _ = atoms_rs_file.write_all(b"use atom::Atom;");

    for (capsname, atom) in config.atoms {
        let Atom { number, symbol, name, mass, diatomic } = atom;

        let rust_atom = format!("
pub const {capsname}: &'static Atom = &Atom {{
    number: {number}, mass: {mass:.5}, symbol: \"{symbol}\", name: \"{name}\", is_diatomic: {diatomic} }};
        ", capsname=capsname, name=name, number=number, mass=mass, symbol=symbol, diatomic=diatomic );

        let _ = atoms_rs_file.write_all( rust_atom.as_bytes() );
    }

    // let _ = atoms_rs_file.write_all(  format!("{:?}", config).as_bytes()  );
}
