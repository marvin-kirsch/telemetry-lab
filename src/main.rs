use std::fmt::Display;
use std::thread::sleep;
use std::time::{Duration, Instant};

#[derive(Copy, Clone)]
struct Reading {
    sim_temperature: f64,
    elapsed_time: Duration,
    sequence_number: i32,
}

impl Display for Reading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "sn: \t{}\ntemp: \t{}\netime: \t{}",
            self.sequence_number,
            self.sim_temperature,
            self.elapsed_time.as_millis()
        )
    }
}

fn main() {
    let start_time = Instant::now();

    let mut readings: Vec<Reading> = vec![];

    let mut simple_reading = Reading {
        sim_temperature: 100.0,
        sequence_number: 0,
        elapsed_time: Duration::from_secs(1),
    };

    for _c in 0..10 {
        sleep(Duration::from_millis(100));
        simple_reading.elapsed_time = start_time.elapsed();
        simple_reading.sequence_number += 1;
        readings.push(simple_reading);
        println!("{}", simple_reading);
    }

    println!("{}", readings.iter().len());
    let sum_temp: f64 = readings.iter().map(|r| r.sim_temperature).sum();
    let average_temp = sum_temp / readings.len() as f64;

    let average_et: Duration = readings.last().unwrap().elapsed_time / readings.len() as u32;

    println!("average_et: {}", average_et.as_millis());
    println!("average_temp: {}", average_temp);
}
