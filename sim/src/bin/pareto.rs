use std::fmt::Write;

use rand::rngs::Rng;

const SEED: [u64; 4] = [
    0x5ac6b27ff90c4d13,
    0x63dc705cd7f0559b,
    0x323c660b0356facf,
    0x7bacd1bfe56ae9f5,
];

const DATA_COUNT: usize = 100000;

fn main() {
    let mut rng = rand::rngs::Xoshiro256Plus::new(SEED);

    let pareto = {
        let shape = 1.8;

        let mean = 1.;
        let scale = mean * (shape - 1.);

        rand::distributions::ParetoII::new(shape, scale)
    };

    let data = std::iter::repeat_with(|| rng.sample(&pareto))
        .take(DATA_COUNT)
        .collect::<Vec<_>>();

    let mut output = String::from("x\n");
    for x in data {
        writeln!(&mut output, "{}", x).unwrap();
    }
    std::fs::write("pareto.csv", output).unwrap();
}