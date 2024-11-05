use num::Float;

use crate::rngs::Rng;

use super::{Distribution, StandardUniformClosedOpen};

pub struct ParetoII<F> {
    pow: F,
    scale: F,
}

impl<F> ParetoII<F>
where
    F: Float,
{
    pub fn new(shape: F, scale: F) -> Self {
        Self {
            pow: -shape.recip(),
            scale,
        }
    }
}

impl<F> Distribution<F> for ParetoII<F>
where
    F: Float,
    StandardUniformClosedOpen: Distribution<F>,
{
    // inverse CDF sampling technique
    fn sample<R>(&self, rng: &mut R) -> F
    where
        R: Rng + ?Sized,
    {
        let u = rng.sample(&StandardUniformClosedOpen);

        // alpha = shape, xm = scale
        // u = F(x) = 1 - (xm / (x + xm)) ^ alpha
        // x = xm ((1-u)^(-1/alpha) - 1)

        self.scale * (u.powf(self.pow) - F::ONE)
    }
}
