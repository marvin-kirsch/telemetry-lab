use std::fmt::Display;
use std::thread::sleep;
use std::time::{Duration, Instant};

struct Reading {
    sim_temperatur: f64,
    elapsed_time: Duration,
    squence_number: i32,
}

impl Display for Reading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "sn: \t{}\ntemp: \t{}\netime: \t{}",
            self.squence_number,
            self.sim_temperatur,
            self.elapsed_time.as_millis()
        )
    }
}

fn main() {
    let start_time = Instant::now();

    let mut simple_reading = Reading {
        sim_temperatur: 100.0,
        squence_number: 0,
        elapsed_time: Duration::from_secs(1),
    };

    for _c in 0..10 {
        sleep(Duration::from_millis(100));
        simple_reading.elapsed_time = Instant::now() - start_time;
        simple_reading.squence_number += 1;
        println!("{}", simple_reading);
    }
}
