use core::cmp::Ordering;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait IntoFloat {
    fn into_f32(self) -> f32;
    fn into_f64(self) -> f64;
}

pub trait Float:
    Sized
    + Copy
    + Neg<Output = Self>
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
{
    // this uses an `as` cast to do the conversion
    // use this when the value has an exact representation.
    // for anything else, prefer to add an associated constant.
    fn from(x: impl IntoFloat) -> Self;

    const ZERO: Self;
    const ONE: Self;
    const PI: Self;

    fn sqrt(self) -> Self;
    fn powf(self, n: Self) -> Self;
    fn recip(self) -> Self;
    fn ln(self) -> Self;
    fn sin_cos(self) -> (Self, Self);
    fn mul_add(self, a: Self, b: Self) -> Self;
    fn abs(self) -> Self;

    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;

    fn total_cmp(&self, other: &Self) -> Ordering;
}

macro_rules! float_impl {
    ($fty:tt, $conv:ident) => {
        impl IntoFloat for $fty {
            fn into_f32(self) -> f32 {
                self as f32
            }

            fn into_f64(self) -> f64 {
                self as f64
            }
        }

        impl Float for $fty {
            fn from(f: impl IntoFloat) -> Self {
                f.$conv()
            }

            const ZERO: Self = 0.;
            const ONE: Self = 1.;
            const PI: Self = core::$fty::consts::PI;

            fn sqrt(self) -> Self {
                self.sqrt()
            }

            fn powf(self, n: Self) -> Self {
                self.powf(n)
            }

            fn recip(self) -> Self {
                self.recip()
            }

            fn ln(self) -> Self {
                self.ln()
            }

            fn sin_cos(self) -> (Self, Self) {
                self.sin_cos()
            }

            fn mul_add(self, a: Self, b: Self) -> Self {
                self.mul_add(a, b)
            }

            fn abs(self) -> Self {
                self.abs()
            }

            fn max(self, other: Self) -> Self {
                self.max(other)
            }

            fn min(self, other: Self) -> Self {
                self.min(other)
            }

            fn total_cmp(&self, other: &Self) -> Ordering {
                self.total_cmp(other)
            }
        }
    };
}

float_impl! { f32, into_f32 }
float_impl! { f64, into_f64 }
