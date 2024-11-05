use crate::rngs::Rng;

mod normal;
mod uniform;
mod exponential;
mod pareto;

pub use normal::{Normal, StandardNormal};
pub use exponential::Exponential;
pub use uniform::{
    StandardUniformClosedOpen, StandardUniformOpenClosed, StandardUniformOpenOpen,
    UniformClosedOpen, UniformOpenClosed, UniformOpenOpen,
};
pub use pareto::ParetoII;

pub trait Distribution<T> {
    fn sample<R>(&self, rng: &mut R) -> T
    where
        R: Rng + ?Sized;
}
