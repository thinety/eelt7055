use std::fmt::Write;
use std::{cell::RefCell, rc::Rc};

use queue::system::QueueSystem;
use rand::rngs::Rng;

fn get_system<R: Rng + 'static>(rng: &Rc<RefCell<R>>, alpha: f64) -> impl QueueSystem<f64> {
    let arrival_interval = {
        let mut arma = {
            let phi = [0.7];
            let theta = [-0.3];
            let std_dev = 3_f64.sqrt();
            let mean = 5.;
            rand::series::Arma::new(phi, theta, std_dev, mean)
        };

        let rng = rng.clone();
        move || rng.borrow_mut().get_next(&mut arma).abs() * alpha
    };

    let serving_time_1 = {
        let dist = {
            let mean = 1.;
            rand::distributions::Exponential::new(mean)
        };

        let rng = rng.clone();
        move || rng.borrow_mut().sample(&dist)
    };

    let serving_time_2 = {
        let dist = {
            let shape = 1.8;

            let mean = 1.;
            let scale = mean * (shape - 1.);

            rand::distributions::ParetoII::new(shape, scale)
        };

        let rng = rng.clone();
        move || rng.borrow_mut().sample(&dist)
    };

    queue::system::Fifo::new(
        Box::new(arrival_interval),
        [Box::new(serving_time_1), Box::new(serving_time_2)],
    )
}

const SEED: [u64; 4] = [
    0x5ac6b27ff90c4d13,
    0x63dc705cd7f0559b,
    0x323c660b0356facf,
    0x7bacd1bfe56ae9f5,
];

const SIMULATION_COUNT: usize = 100;
const WARMUP_COUNT: usize = 10000;
const DATA_COUNT: usize = 100000;

fn main() {
    let mut output = String::new();

    for mult in 1..=10 {
        write!(
            &mut output,
            "queue_time_alpha{:02}{}",
            mult,
            if mult < 10 { "," } else { "\n" },
        )
        .unwrap();
    }

    for i in 0..SIMULATION_COUNT {
        for mult in 1..=10 {
            let mut rng = rand::rngs::Xoshiro256Plus::new(SEED);
            for _ in 0..i {
                rng.jump();
            }
            let rng = Rc::new(RefCell::new(rng));

            let alpha = 0.1 * mult as f64;

            let mut system = get_system(&rng, alpha);

            let queue_times = std::iter::repeat_with(|| {
                let client = system.step();
                client.serve_instant - client.arrival_instant
            })
            .skip(WARMUP_COUNT)
            .take(DATA_COUNT)
            .collect::<Vec<_>>();

            let average_queue_time = queue_times.iter().sum::<f64>() / DATA_COUNT as f64;

            write!(
                &mut output,
                "{}{}",
                average_queue_time,
                if mult < 10 { "," } else { "\n" },
            )
            .unwrap();
        }
    }

    std::fs::write("sys.csv", output).unwrap();
}
