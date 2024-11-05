use num::Float;

use crate::rngs::Rng;

use super::Distribution;

pub struct StandardUniformClosedOpen;

pub struct StandardUniformOpenClosed;

pub struct StandardUniformOpenOpen;

macro_rules! continuous_uniform_impl {
    ($fty:ty, $uty:ty, $gen:ident, $total_bits:expr, $mantissa_bits:expr) => {
        impl Distribution<$fty> for StandardUniformClosedOpen {
            fn sample<R>(&self, rng: &mut R) -> $fty
            where
                R: Rng + ?Sized,
            {
                let value = rng.$gen() >> ($total_bits - $mantissa_bits);

                let scale = 1.0 / (((1 as $uty) << $mantissa_bits) as $fty);

                scale * (value as $fty)
            }
        }

        impl Distribution<$fty> for StandardUniformOpenClosed {
            fn sample<R>(&self, rng: &mut R) -> $fty
            where
                R: Rng + ?Sized,
            {
                let value = rng.$gen() >> ($total_bits - $mantissa_bits);

                let scale = 1.0 / (((1 as $uty) << $mantissa_bits) as $fty);

                scale * ((value + 1) as $fty)
            }
        }

        impl Distribution<$fty> for StandardUniformOpenOpen {
            fn sample<R>(&self, rng: &mut R) -> $fty
            where
                R: Rng + ?Sized,
            {
                let value = rng.$gen() >> ($total_bits - $mantissa_bits);

                let scale = 1.0 / (((1 as $uty) << $mantissa_bits) as $fty);

                scale * ((value | 1) as $fty)
            }
        }
    };
}

continuous_uniform_impl! { f32, u32, next_u32, 32, 24 }
continuous_uniform_impl! { f64, u64, next_u64, 64, 53 }

pub struct UniformClosedOpen<F> {
    start: F,
    range: F,
}

pub struct UniformOpenClosed<F> {
    start: F,
    range: F,
}

pub struct UniformOpenOpen<F> {
    start: F,
    range: F,
}

impl<F> UniformClosedOpen<F>
where
    F: Float,
{
    pub fn new(start: F, end: F) -> Self {
        Self {
            start,
            range: end - start,
        }
    }
}

impl<F> UniformOpenClosed<F>
where
    F: Float,
{
    pub fn new(start: F, end: F) -> Self {
        Self {
            start,
            range: end - start,
        }
    }
}

impl<F> UniformOpenOpen<F>
where
    F: Float,
{
    pub fn new(start: F, end: F) -> Self {
        Self {
            start,
            range: end - start,
        }
    }
}

impl<F> From<core::ops::Range<F>> for UniformClosedOpen<F>
where
    F: Float,
{
    fn from(range: core::ops::Range<F>) -> Self {
        Self::new(range.start, range.end)
    }
}

impl<F> From<core::ops::Range<F>> for UniformOpenClosed<F>
where
    F: Float,
{
    fn from(range: core::ops::Range<F>) -> Self {
        Self::new(range.start, range.end)
    }
}

impl<F> From<core::ops::Range<F>> for UniformOpenOpen<F>
where
    F: Float,
{
    fn from(range: core::ops::Range<F>) -> Self {
        Self::new(range.start, range.end)
    }
}

impl<F> Distribution<F> for UniformClosedOpen<F>
where
    F: Float,
    StandardUniformClosedOpen: Distribution<F>,
{
    fn sample<R>(&self, rng: &mut R) -> F
    where
        R: Rng + ?Sized,
    {
        let x = rng.sample(&StandardUniformClosedOpen);

        x.mul_add(self.range, self.start)
    }
}

impl<F> Distribution<F> for UniformOpenClosed<F>
where
    F: Float,
    StandardUniformOpenClosed: Distribution<F>,
{
    fn sample<R>(&self, rng: &mut R) -> F
    where
        R: Rng + ?Sized,
    {
        let x = rng.sample(&StandardUniformOpenClosed);

        x.mul_add(self.range, self.start)
    }
}

impl<F> Distribution<F> for UniformOpenOpen<F>
where
    F: Float,
    StandardUniformOpenOpen: Distribution<F>,
{
    fn sample<R>(&self, rng: &mut R) -> F
    where
        R: Rng + ?Sized,
    {
        let x = rng.sample(&StandardUniformOpenOpen);

        x.mul_add(self.range, self.start)
    }
}
