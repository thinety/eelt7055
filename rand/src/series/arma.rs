use num::Float;

use crate::distributions::{Distribution, StandardNormal};
use crate::rngs::Rng;

use super::TimeSeries;

pub struct Arma<F, const P: usize, const Q: usize> {
    phi: [F; P],
    z: [F; P],
    theta: [F; Q],
    e: [F; Q],
    std_dev: F,
    mean: F,
}

impl<F, const P: usize, const Q: usize> Arma<F, P, Q>
where
    F: Float,
{
    pub fn new(phi: [F; P], theta: [F; Q], std_dev: F, mean: F) -> Self {
        Self {
            phi,
            z: [F::ZERO; P],
            theta,
            e: [F::ZERO; Q],
            std_dev,
            mean,
        }
    }
}

impl<F, const P: usize, const Q: usize> TimeSeries<F> for Arma<F, P, Q>
where
    F: Float,
    StandardNormal: Distribution<F>,
{
    fn get_next<R>(&mut self, rng: &mut R) -> F
    where
        R: Rng + ?Sized,
    {
        let mut new_z = F::ZERO;
        let new_e = rng.sample(&StandardNormal) * self.std_dev;

        for i in 0..P {
            new_z += self.phi[i] * self.z[i];
        }
        new_z += new_e;
        for i in 0..Q {
            new_z += self.theta[i] * self.e[i];
        }

        for i in (1..P).rev() {
            self.z[i] = self.z[i - 1];
        }
        self.z[0] = new_z;

        for i in (1..Q).rev() {
            self.e[i] = self.e[i - 1];
        }
        self.e[0] = new_e;

        new_z + self.mean
    }
}
