#[macro_use]
extern crate serde_derive;
extern crate toml;


mod types;
use types::{ AtomNumber, AtomGroup, AtomMass };

use std::fs::File;
use std::collections::HashMap;
use std::io::*;


#[derive(Debug, Serialize, Deserialize)]
struct Config {
    pub atoms: HashMap<String, Atom>
}

#[derive(Debug, Serialize, Deserialize)]
struct Atom {
    pub number: AtomNumber,
    pub symbol: String,
    pub name: String,
    pub group: AtomGroup,
    pub mass: AtomMass,
    pub diatomic: bool
}


#[allow(dead_code)]
/// Write an example TOML file to the data_atoms.rs file
fn write(mut atoms_rs_file: &File) {
    let mut atoms: HashMap<String, Atom> = HashMap::new();

    // Add example atom
    atoms.insert("hydrogen".to_owned(), Atom {
        number: 1,
        symbol: "H".to_owned(),
        name: "hydrogen".to_owned(),
        group: 1,
        mass: 1.008,
        diatomic: true
    });

    // Generate config
    let config = Config {
        atoms: atoms
    };

    // Convert to TOML
    let config_string = toml::to_string(&config).unwrap();

    // Write TOML to file
    atoms_rs_file.write_all( config_string.as_bytes() ).unwrap();
}


#[allow(dead_code)]
/// Reads the data_atoms.toml file, converts it to the appropriate format
/// and writes it to the atoms.rs file
fn read_and_write(mut atoms_toml_file: &File, mut atoms_rs_file: &File) {
    // Read TOML file
    let mut atoms_toml = String::new();
    let _ = atoms_toml_file.read_to_string(&mut atoms_toml);

    // Convert to config struct
    let config: Config;
    match toml::from_str(&atoms_toml) {
        Ok(x) => { config = x },
        Err(e) => { panic!("{:?}", e) }
    }

    // Write header to file
    atoms_rs_file.write_all(b"use atom::Atom;").unwrap();

    // Convert items from TOML file to RS syntax
    for (capsname, atom) in config.atoms {
        let Atom { number, symbol, name, group, mass, diatomic } = atom;

        let rust_atom = format!("
pub const {capsname}: Atom = Atom {{
    number: {number}, mass: {mass:.5}, symbol: \"{symbol}\", name: \"{name}\", group: {group:?}, is_diatomic: {diatomic} }};
",
    capsname = capsname, name = name, number = number, mass = mass,
    symbol = symbol, group = group, diatomic = diatomic );

        // Append to file
        atoms_rs_file.write_all( rust_atom.as_bytes() ).unwrap();
    }
}


fn main() {
    let mut atoms_toml_file = File::open("src/data_atoms.toml").unwrap();
    let mut atoms_rs_file = File::create("src/data_atoms.rs").unwrap();

    // NOTE: For debugging only:
    // write(&mut atoms_rs_file);

    read_and_write(&mut atoms_toml_file, &mut atoms_rs_file);
}
