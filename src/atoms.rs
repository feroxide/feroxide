use atom::Atom;

// Using data from https://en.wikipedia.org/wiki/Periodic_table_(large_cells)

pub const HYDROGEN: &'static Atom = &Atom {
    number: 1, mass: 1.008, symbol: "H", name: "hydrogen", is_diatomic: true };

pub const HELIUM: &'static Atom = &Atom {
     number: 2, mass: 4.002602, symbol: "He", name: "helium", is_diatomic: false };

pub const LITHIUM: &'static Atom = &Atom {
     number: 3, mass: 6.94, symbol: "Li", name: "lithium", is_diatomic: false };

pub const BERYLLIUM: &'static Atom = &Atom {
     number: 4, mass: 9.0121831, symbol: "Be", name: "beryllium", is_diatomic: false };

pub const BORON: &'static Atom = &Atom {
     number: 5, mass: 10.81, symbol: "B", name: "boron", is_diatomic: false };

pub const CARBON: &'static Atom = &Atom {
     number: 6, mass: 12.011, symbol: "C", name: "carbon", is_diatomic: false };

pub const NITROGEN: &'static Atom = &Atom {
     number: 7, mass: 14.007, symbol: "N", name: "nitrogen", is_diatomic: true };

pub const OXYGEN: &'static Atom = &Atom {
     number: 8, mass: 15.999, symbol: "O", name: "oxygen", is_diatomic: true };

pub const FLUORINE: &'static Atom = &Atom {
     number: 9, mass: 18.998403163, symbol: "F", name: "fluorine", is_diatomic: true };

pub const NEON: &'static Atom = &Atom {
     number: 10, mass: 20.1797, symbol: "Ne", name: "neon", is_diatomic: false };

pub const SODIUM: &'static Atom = &Atom {
     number: 11, mass: 22.98976928, symbol: "Na", name: "sodium", is_diatomic: false };

pub const MAGNESIUM: &'static Atom = &Atom {
     number: 12, mass: 24.305, symbol: "Mg", name: "magnesium", is_diatomic: false };

pub const ALUMINIUM: &'static Atom = &Atom {
     number: 13, mass: 26.9815385, symbol: "Al", name: "aluminium", is_diatomic: false };

pub const SILICON: &'static Atom = &Atom {
    number: 14, mass: 28.085, symbol: "Si", name: "silicon", is_diatomic: false };

pub const PHOSPHORUS: &'static Atom = &Atom {
    number: 15, mass: 30.973761998, symbol: "P", name: "phosphorus", is_diatomic: false };

pub const SULFUR: &'static Atom = &Atom {
     number: 16, mass: 32.06, symbol: "S", name: "sulfur", is_diatomic: false };

pub const CHLORINE: &'static Atom = &Atom {
     number: 17, mass: 35.45, symbol: "Cl", name: "chlorine", is_diatomic: true };



pub const BROMINE: &'static Atom = &Atom {
     number: 35, mass: 79.904, symbol: "Br", name: "bromine", is_diatomic: true };

pub const IODINE: &'static Atom = &Atom {
     number: 53, mass: 126.90447, symbol: "I", name: "iodine", is_diatomic: true };
