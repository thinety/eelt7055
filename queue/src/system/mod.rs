mod fifo;
pub use fifo::Fifo;

pub struct Client<F> {
    pub arrival_instant: F,
    pub serve_instant: F,
    pub server: usize,
    pub departure_instant: F,
}

pub trait QueueSystem<F> {
    fn step(&mut self) -> Client<F>;
}
