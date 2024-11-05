use num::Float;

use crate::rngs::Rng;

use super::{Distribution, StandardUniformOpenClosed};

pub struct Exponential<F> {
    mean: F,
}

impl<F> Exponential<F>
where
    F: Float,
{
    pub fn new(mean: F) -> Self {
        Self { mean }
    }
}

impl<F> Distribution<F> for Exponential<F>
where
    F: Float,
    StandardUniformOpenClosed: Distribution<F>,
{
    // inverse CDF sampling technique
    fn sample<R>(&self, rng: &mut R) -> F
    where
        R: Rng + ?Sized,
    {
        let u = rng.sample(&StandardUniformOpenClosed);

        // u = F(x) = 1 - e^(-x/mu)
        // x = -mu * ln(1 - u)

        -self.mean * u.ln()
    }
}
