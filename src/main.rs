use std::fmt::Display;
use std::time::Duration;

struct Reading {
    sim_temperatur: i64,
    elapsed_time: Duration,
    squence_number: i64,
}

impl Display for Reading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.squence_number,
            self.sim_temperatur,
            self.elapsed_time.as_millis()
        )
    }
}

fn main() {
    let mut simple_reading = Reading {
        sim_temperatur: 100,
        squence_number: 1,
        elapsed_time: Duration::from_secs(1),
    };

    for c in 1..4 {
        println!("{}", simple_reading);
    }
}
