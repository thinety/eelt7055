use std::{cell::RefCell, rc::Rc};

use queue::system::QueueSystem;
use rand::rngs::Rng;

const SEED: [u64; 4] = [
    0xce124f618403c393,
    0x28d53c991db633b3,
    0x84e1e11761ad8d8f,
    0x3d51155d3a5e4243,
];

fn system1<R: Rng + 'static>(rng: &Rc<RefCell<R>>) -> impl QueueSystem<f64> {
    let arrival_interval = {
        let mut arma = {
            let phi = [0.7];
            let theta = [-0.3];
            let mean = 5.;
            let std_dev = 3_f64.sqrt();
            rand::series::Arma::new(phi, theta, mean, std_dev)
        };

        let alpha = 0.1;

        let rng = rng.clone();
        move || rng.borrow_mut().get_next(&mut arma).abs() * alpha
    };

    let serving_time_1 = {
        let mean = 1.;
        let dist = rand::distributions::Exponential::new(mean);

        let rng = rng.clone();
        move || rng.borrow_mut().sample(&dist)
    };

    let serving_time_2 = {
        let shape = 1.8;

        let mean = 1.;
        let scale = mean * (shape - 1.);

        let dist = rand::distributions::ParetoII::new(shape, scale);

        let rng = rng.clone();
        move || rng.borrow_mut().sample(&dist)
    };

    queue::system::Fifo::new(
        Box::new(arrival_interval),
        [Box::new(serving_time_1), Box::new(serving_time_2)],
    )
}

fn system2<R: Rng + 'static>(rng: &Rc<RefCell<R>>) -> impl QueueSystem<f64> {
    let arrival_interval = {
        let mean = 5.;
        let dist = rand::distributions::Exponential::new(mean);

        let rng = rng.clone();
        move || rng.borrow_mut().sample(&dist)
    };

    let serving_time_1 = {
        let mean = 1.;
        let dist = rand::distributions::Exponential::new(mean);

        let rng = rng.clone();
        move || rng.borrow_mut().sample(&dist)
    };

    let serving_time_2 = {
        let mean = 1.;
        let dist = rand::distributions::Exponential::new(mean);

        let rng = rng.clone();
        move || rng.borrow_mut().sample(&dist)
    };

    queue::system::Fifo::new(
        Box::new(arrival_interval),
        [Box::new(serving_time_1), Box::new(serving_time_2)],
    )
}

fn main() {
    let rng = rand::rngs::Xoshiro256Plus::new(SEED);
    let rng = Rc::new(RefCell::new(rng));

    let mut system = system1(&rng);

    let mut output = String::from("arrival_interval,queue_time,service_time,server\n");

    let mut last_arrival_instant = 0.;

    for _ in 0..1000000 {
        let client = system.step();

        let arrival_interval = client.arrival_instant - last_arrival_instant;
        last_arrival_instant = client.arrival_instant;
        let queue_time = client.serve_instant - client.arrival_instant;
        let service_time = client.departure_instant - client.serve_instant;

        output += &format!(
            "{},{},{},{}\n",
            arrival_interval, queue_time, service_time, client.server
        );
    }

    std::fs::write("intervals.csv", output).unwrap();
}
