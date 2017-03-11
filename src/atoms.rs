use atom::Atom;

// Using data from https://en.wikipedia.org/wiki/Periodic_table_(large_cells)

pub const HYDROGEN: &'static Atom = &Atom { atom_number: 1, mass: 1.008, symbol: "H", name: "hydrogen" };
pub const HELIUM: &'static Atom = &Atom { atom_number: 2, mass: 4.002602, symbol: "He", name: "helium" };
pub const LITHIUM: &'static Atom = &Atom { atom_number: 3, mass: 6.94, symbol: "Li", name: "lithium" };
pub const BERYLLIUM: &'static Atom = &Atom { atom_number: 4, mass: 9.0121831, symbol: "Be", name: "beryllium" };
pub const BORON: &'static Atom = &Atom { atom_number: 5, mass: 10.81, symbol: "B", name: "boron" };
pub const CARBON: &'static Atom = &Atom { atom_number: 6, mass: 12.011, symbol: "C", name: "carbon" };
pub const NITROGEN: &'static Atom = &Atom { atom_number: 7, mass: 14.007, symbol: "N", name: "nitrogen" };
pub const OXYGEN: &'static Atom = &Atom { atom_number: 8, mass: 15.999, symbol: "O", name: "oxygen" };
pub const FLUORINE: &'static Atom = &Atom { atom_number: 9, mass: 18.998403163, symbol: "F", name: "fluorine" };
pub const NEON: &'static Atom = &Atom { atom_number: 10, mass: 20.1797, symbol: "Ne", name: "neon" };
pub const SODIUM: &'static Atom = &Atom { atom_number: 11, mass: 22.98976928, symbol: "Na", name: "sodium" };
pub const MAGNESIUM: &'static Atom = &Atom { atom_number: 12, mass: 24.305, symbol: "Mg", name: "magnesium" };
pub const ALUMINIUM: &'static Atom = &Atom { atom_number: 13, mass: 26.9815385, symbol: "Al", name: "aluminium" };

pub const SULFUR: &'static Atom = &Atom { atom_number: 16, mass: 32.06, symbol: "S", name: "sulfur" };
pub const CHLORINE: &'static Atom = &Atom { atom_number: 17, mass: 35.45, symbol: "Cl", name: "chlorine" };
