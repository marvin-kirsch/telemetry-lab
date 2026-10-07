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

#[derive(Copy, Clone, Debug)]
struct TemperatureSummary {
    count: usize,
    min: f64,
    max: f64,
    average: f64,
}

impl Display for TemperatureSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "count: \t{}\nmin: \t{}\nmax: \t{}\nav: \t{}",
            self.count, self.min, self.max, self.average,
        )
    }
}

fn summarize(readings: &[Reading]) -> Option<TemperatureSummary> {
    if readings.is_empty() {
        return None;
    }

    let count = readings.len();
    let sum: f64 = readings.iter().map(|r| r.sim_temperature).sum();

    Some(TemperatureSummary {
        count,
        min: readings
            .iter()
            .map(|r| r.sim_temperature)
            .min_by(f64::total_cmp)?,
        max: readings
            .iter()
            .map(|r| r.sim_temperature)
            .max_by(f64::total_cmp)?,
        average: sum / count as f64,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut test_readings: Vec<Reading> = vec![];
        let result = summarize(&test_readings);
        assert_eq!(result, None);
    }

    #[test]
    fn one_element() {
        let mut test_readings Vec<Reading> = vec![];
        let test_reading: Reading = {
            sim_temperature:,

        }
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

    let average_et: Duration = (readings.last().unwrap().elapsed_time
        - readings.first().unwrap().elapsed_time)
        / (readings.len() as u32 - 1);
    println!("average_et: {}", average_et.as_millis());

    let summary: TemperatureSummary =
        summarize(&readings).expect("If there is nothing, just write nothing?");
    println!("{}", summary);
}
