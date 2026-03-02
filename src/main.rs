use core::f64;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Result};
use std::path::Path;

struct Info {
    min: f64,
    count: usize,
    total: f64,
    max: f64,
}

impl Default for Info {
    fn default() -> Self {
        Info {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
            count: 0,
            total: 0.0,
        }
    }
}

fn main() {
    // do we want it to take a str slice? can change later
    let mut map: HashMap<String, Info> = HashMap::with_capacity(10_000);

    // read the data in the file line by line
    let file = File::open("../data/measurements.txt").unwrap();
    // in the future, allocate a buffer with_capacity = to the bytes in the file
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let (station, temperature) = split_lines(&line.unwrap());

        let stats = map.entry(station).or_default();
        stats.min = stats.min.min(temperature);
        stats.count += 1;
        stats.total += temperature;
        stats.max = stats.max.max(temperature);
    }

    let mut sorted: Vec<(String, Info)> = map.into_iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));

    print!("{{");
    for (station, info) in sorted {
        let mean = info.total / info.count as f64;

        print!("{station} = {}/{}/{}", info.min, mean, info.max);

        print!(", ")
    }

    print!("}}");
}

// takes a line from the file and returns a tuple of String, Temperature
fn split_lines(line: &str) -> (String, f64) {
    let mut parts = line.split(';');

    let station = parts.next().unwrap().to_string();
    let temp_str = parts.next().unwrap();
    let temp: f64 = temp_str.trim().parse::<f64>().unwrap();

    return (station, temp);
}
