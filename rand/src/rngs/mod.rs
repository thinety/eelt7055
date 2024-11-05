use crate::{distributions::Distribution, series::TimeSeries};

mod xoshiro128plus;
mod xoshiro128plusplus;
mod xoshiro256plus;
mod xoshiro256plusplus;

pub use xoshiro128plus::Xoshiro128Plus;
pub use xoshiro128plusplus::Xoshiro128PlusPlus;
pub use xoshiro256plus::Xoshiro256Plus;
pub use xoshiro256plusplus::Xoshiro256PlusPlus;

pub trait Rng {
    fn fill_bytes(&mut self, buf: &mut [u8]);

    fn next_u32(&mut self) -> u32 {
        let mut buf = [0; 4];
        self.fill_bytes(&mut buf);
        u32::from_be_bytes(buf)
    }

    fn next_u64(&mut self) -> u64 {
        let mut buf = [0; 8];
        self.fill_bytes(&mut buf);
        u64::from_be_bytes(buf)
    }

    fn sample<D, T>(&mut self, distribution: &D) -> T
    where
        D: Distribution<T>,
    {
        distribution.sample(self)
    }

    fn get_next<S, T>(&mut self, series: &mut S) -> T
    where
        S: TimeSeries<T>,
    {
        series.get_next(self)
    }
}

pub fn fill_from_u32<F>(mut buf: &mut [u8], mut next: F)
where
    F: FnMut() -> u32,
{
    while buf.len() >= 4 {
        let (l, r) = buf.split_at_mut(4);
        buf = r;

        let chunk = next().to_be_bytes();
        l.copy_from_slice(&chunk);
    }

    let n = buf.len();
    if n > 0 {
        let chunk = next().to_be_bytes();
        buf.copy_from_slice(&chunk[..n]);
    }
}

pub fn fill_from_u64<F>(mut buf: &mut [u8], mut next: F)
where
    F: FnMut() -> u64,
{
    while buf.len() >= 8 {
        let (l, r) = buf.split_at_mut(8);
        buf = r;

        let chunk = next().to_be_bytes();
        l.copy_from_slice(&chunk);
    }

    let n = buf.len();
    if n > 0 {
        let chunk = next().to_be_bytes();
        buf.copy_from_slice(&chunk[..n]);
    }
}
