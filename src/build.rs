#[macro_use]
extern crate serde_derive;
extern crate toml;


mod types;
use types::*;

use std::io::prelude::*;
use std::fs::File;

use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
struct Config {
    pub atoms: HashMap<String, Atom>
}

#[derive(Serialize, Deserialize, Debug)]
struct Atom {
    pub number: AtomNumber,
    pub symbol: String,
    pub name: String,
    pub group: AtomGroup,
    pub mass: AtomMass,
    pub diatomic: bool
}


fn main() {
    let mut atoms_rs_file = File::create("src/data_atoms.rs").unwrap();
    let mut atoms_toml_file = File::open("src/data_atoms.toml").unwrap();

    /*
    let mut atoms: HashMap<String, Atom> = HashMap::new();
    atoms.insert("hydrogen".to_owned(), Atom {
        number: 1,
        symbol: "H".to_owned(),
        name: "hydrogen".to_owned(),
        group: AtomGroup::I as u8,
        mass: 1.008,
        diatomic: true
    });

    let config: Config = Config {
        atoms: atoms
    };

    atoms_rs_file.write_all( toml::to_string(&config).unwrap().as_bytes() );

    return;
    */

    let mut atoms_toml = String::new();
    let _ = atoms_toml_file.read_to_string(&mut atoms_toml);

    let config: Config;
    match toml::from_str(&atoms_toml) {
        Ok(x) => { config = x },
        Err(e) => { panic!("{:?}", e) }
    }

    let _ = atoms_rs_file.write_all(b"use atom::Atom;");

    for (capsname, atom) in config.atoms {
        let Atom { number, symbol, name, group, mass, diatomic } = atom;

        let rust_atom = format!("
pub const {capsname}: &'static Atom = &Atom {{
    number: {number}, mass: {mass:.5}, symbol: \"{symbol}\", name: \"{name}\", group: {group:?}, is_diatomic: {diatomic} }};
        ", capsname=capsname, name=name, number=number, mass=mass, symbol=symbol, group=group, diatomic=diatomic );

        let _ = atoms_rs_file.write_all( rust_atom.as_bytes() );
    }
}
