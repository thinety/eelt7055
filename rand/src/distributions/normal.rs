use num::Float;

use crate::rngs::Rng;

use super::{Distribution, StandardUniformClosedOpen, StandardUniformOpenClosed};

pub struct StandardNormal;

impl<F> Distribution<F> for StandardNormal
where
    F: Float,
    StandardUniformClosedOpen: Distribution<F>,
    StandardUniformOpenClosed: Distribution<F>,
{
    // Box-Muller transform
    fn sample<R>(&self, rng: &mut R) -> F
    where
        R: Rng + ?Sized,
    {
        let u1 = rng.sample(&StandardUniformOpenClosed);
        let u2 = rng.sample(&StandardUniformClosedOpen);

        let r = (F::from(-2.0) * u1.ln()).sqrt();
        let (sint, cost) = (F::from(2.0) * F::PI * u2).sin_cos();

        // discard the other sample
        let (z1, _z2) = (r * cost, r * sint);

        z1
    }
}

pub struct Normal<F> {
    mean: F,
    std_dev: F,
}

impl<F> Normal<F>
where
    F: Float,
{
    pub fn new(mean: F, std_dev: F) -> Self {
        Self { mean, std_dev }
    }
}

impl<F> Distribution<F> for Normal<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
{
    fn sample<R>(&self, rng: &mut R) -> F
    where
        R: Rng + ?Sized,
    {
        let z = rng.sample(&StandardNormal);
        z.mul_add(self.std_dev, self.mean)
    }
}
