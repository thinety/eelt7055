use core::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

use num::Float;

use super::{Client, QueueSystem};

struct Server<F> {
    index: usize,
    instant: F,
}

impl<F> PartialEq for Server<F>
where
    F: Float,
{
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<F> Eq for Server<F> where F: Float {}

impl<F> PartialOrd for Server<F>
where
    F: Float,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<F> Ord for Server<F>
where
    F: Float,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.instant
            .total_cmp(&other.instant)
            .then(self.index.cmp(&other.index))
    }
}

pub struct Fifo<F, const M: usize> {
    last_arrival_instant: F,
    arrival_interval: Box<dyn FnMut() -> F>,
    servers: BinaryHeap<Reverse<Server<F>>>,
    serving_times: [Box<dyn FnMut() -> F>; M],
}

impl<F, const M: usize> Fifo<F, M>
where
    F: Float,
{
    pub fn new(
        arrival_interval: Box<dyn FnMut() -> F>,
        serving_times: [Box<dyn FnMut() -> F>; M],
    ) -> Self {
        let mut servers = BinaryHeap::new();
        for i in 0..M {
            servers.push(Reverse(Server {
                index: i,
                instant: F::ZERO,
            }));
        }

        Self {
            last_arrival_instant: F::ZERO,
            arrival_interval,
            servers,
            serving_times,
        }
    }
}

impl<F, const M: usize> QueueSystem<F> for Fifo<F, M>
where
    F: Float,
{
    fn step(&mut self) -> Client<F> {
        let arrival_instant = self.last_arrival_instant + (self.arrival_interval)();

        let Reverse(server) = self
            .servers
            .pop()
            .expect("there should be at least one server");
        let serve_instant = server.instant.max(arrival_instant);

        let departure_instant = serve_instant + (self.serving_times[server.index])();

        self.last_arrival_instant = arrival_instant;

        self.servers.push(Reverse(Server {
            index: server.index,
            instant: departure_instant,
        }));

        Client {
            arrival_instant,
            serve_instant,
            server: server.index,
            departure_instant,
        }
    }
}
