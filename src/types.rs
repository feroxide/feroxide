#![allow(dead_code)]

pub type AtomNumber = u8; // should fit 0 to 118+
pub type AtomGroup = u8; // should fit 1 to 18
pub type AtomMass = f32; // sould fit 0.0 to 294.0+, with normal precision [ gram / mol ]
pub type Energy = f64; // should fit 0.0 to 1e5+, with high precision [ Joule ]
pub type IonCharge = i8; // should fit -7 to 7
pub type Moles = f64; // should fit 0.0 to 1e5+, with high precision [ mol ]
pub type Mass = f64; // should fit 0.0 to 1e5+, with high precision [ gram ]
pub type SEP = f32; // should fit -5.0 to 5.0, with normal precision [ volt ]
