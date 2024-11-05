use crate::rngs::Rng;

mod arma;
pub use arma::Arma;

pub trait TimeSeries<T> {
    fn get_next<R>(&mut self, rng: &mut R) -> T
    where
        R: Rng + ?Sized;
}
