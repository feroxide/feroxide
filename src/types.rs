#![allow(dead_code)]

pub type AtomNumber = u8; // should fit 0 to 118 (possibly higher)
pub type AtomGroup = u8; // should fit 1 to 18
pub type AtomMass = f32; // sould fit 0.0 to 294.0 (possibly higher), with normal precision [ g/mol ]
pub type SEP = f32; // should fit -5 to +5, with normal precision [ V ]

pub type IonCharge = i8; // should fit -7 to 7
pub type Energy = f64; // should fit 0.0 to 1e5, with high precision  [ J ]
pub type Moles = f64; // should fit 0.0 to 1e5, with high precision [ moles ]
pub type Mass = f64; // should fit 0.0 to 1e5, with high precision [ g ]
