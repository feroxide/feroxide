use std::cmp;
use std::fmt;
use std::ops;

macro_rules! allow_conversion {
    ($class:ident, $type:ident) => {
        impl From<$type> for $class {
            fn from(x: $type) -> Self {
                $class(x)
            }
        }

        impl From<$class> for $type {
            fn from(x: $class) -> Self {
                x.0
            }
        }
    };
}

macro_rules! overload_operators {
    ($class:ident, $type:ident) => {
        impl Eq for $class {}

        impl Ord for $class {
            fn cmp(&self, rhs: &Self) -> cmp::Ordering {
                if self.0 < rhs.0 {
                    cmp::Ordering::Less
                } else if self.0 > rhs.0 {
                    cmp::Ordering::Greater
                } else {
                    cmp::Ordering::Equal
                }
            }
        }

        impl ops::Add<$class> for $class {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                $class::from(self.0 + rhs.0)
            }
        }

        impl ops::AddAssign<$class> for $class {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0
            }
        }

        impl ops::Sub<$class> for $class {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                $class::from(self.0 - rhs.0)
            }
        }

        impl ops::SubAssign<$class> for $class {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0
            }
        }

        impl ops::Mul<$type> for $class {
            type Output = Self;

            fn mul(self, rhs: $type) -> Self {
                $class::from(self.0 * rhs)
            }
        }

        impl ops::MulAssign<$type> for $class {
            fn mul_assign(&mut self, rhs: $type) {
                self.0 *= rhs
            }
        }

        impl ops::Div<$type> for $class {
            type Output = Self;

            fn div(self, rhs: $type) -> Self {
                $class::from(self.0 / rhs)
            }
        }

        impl ops::DivAssign<$type> for $class {
            fn div_assign(&mut self, rhs: $type) {
                self.0 /= rhs
            }
        }

        impl ops::Rem<$type> for $class {
            type Output = Self;

            fn rem(self, rhs: $type) -> Self {
                Self::from(self.0 % rhs)
            }
        }

        impl ops::RemAssign<$type> for $class {
            fn rem_assign(&mut self, rhs: $type) {
                self.0 %= rhs
            }
        }
    };
}

macro_rules! allow_display {
    ($class:ident) => {
        impl fmt::Display for $class {
            fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }
    };
}

/// should fit 1 to 18 [  ]
// pub type AtomGroup = u8;
pub type AtomGroupType = u8;
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub struct AtomGroup(pub AtomGroupType);
allow_conversion!(AtomGroup, AtomGroupType);
overload_operators!(AtomGroup, AtomGroupType);
allow_display!(AtomGroup);

/// should fit 0.0 to 294.0+, with normal precision [ gram / mol ]
// pub type AtomMass = f32;
pub type AtomMassType = f32;
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct AtomMass(pub AtomMassType);
allow_conversion!(AtomMass, AtomMassType);
overload_operators!(AtomMass, AtomMassType);
allow_display!(AtomMass);

/// should fit 0 to 118+ [  ]
// pub type AtomNumber = u8;
pub type AtomNumberType = u8;
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub struct AtomNumber(pub AtomNumberType);
allow_conversion!(AtomNumber, AtomNumberType);
overload_operators!(AtomNumber, AtomNumberType);
allow_display!(AtomNumber);

/// should fit 0.0 to 1e5+, with high precision [ Joule ]
// pub type Energy = f64;
pub type EnergyType = f64;
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Energy(pub EnergyType);
allow_conversion!(Energy, EnergyType);
overload_operators!(Energy, EnergyType);
allow_display!(Energy);

/// should fit -7 to 7 [  ]
// pub type AtomCharge = i8;
pub type AtomChargeType = i8;
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub struct AtomCharge(pub AtomChargeType);
allow_conversion!(AtomCharge, AtomChargeType);
overload_operators!(AtomCharge, AtomChargeType);
allow_display!(AtomCharge);

/// should fit 0.0 to 1e5+, with high precision [ gram ]
// pub type Mass = f64;
pub type MassType = f64;
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Mass(pub MassType);
allow_conversion!(Mass, MassType);
overload_operators!(Mass, MassType);
allow_display!(Mass);

/// should fit 0.0 to 1e5+, with high precision [ mol ]
// pub type Moles = f64;
pub type MolesType = f64;
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Moles(pub MolesType);
allow_conversion!(Moles, MolesType);
overload_operators!(Moles, MolesType);
allow_display!(Moles);

/// should fit -5.0 to 5.0, with normal precision [ volt ]
// pub type SEP = f32;
pub type SEPType = f32;
#[derive(Debug, Clone, PartialEq, PartialOrd, Copy)]
pub struct SEP(pub SEPType);
allow_conversion!(SEP, SEPType);
overload_operators!(SEP, SEPType);
allow_display!(SEP);

/// should fit -4000.0 to 2000.0, with normal precision [ kJ/mol ]
// pub type SEP = f32;
pub type SEFType = f32;
#[derive(Debug, Clone, PartialEq, PartialOrd, Copy)]
pub struct SEF(pub SEFType);
allow_conversion!(SEF, SEFType);
overload_operators!(SEF, SEFType);
allow_display!(SEF);
