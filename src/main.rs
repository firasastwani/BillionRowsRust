use core::f64;
use memmap2::Mmap;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
        let line = line.unwrap();
        let (station, temp) = split_lines(&line);

        // only allcate on new entries
        if let Some(stats) = map.get_mut(station) {
            stats.min = stats.min.min(temp);
            stats.count += 1;
            stats.total += temp;
            stats.max = stats.max.max(temp);
        } else {
            let mut stats = Info::default();
            stats.min = temp;
            stats.max = temp;
            stats.count = 1;
            stats.total = temp;
            map.insert(station.to_owned(), stats);
        }
    }

    let mut sorted: Vec<(String, Info)> = map.into_iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));

    print!("{{");
    for (station, info) in sorted {
        let mean = info.total / info.count as f64;

        print!("{station} = {}/{:.1}/{}", info.min, mean, info.max);

        print!(", ")
    }

    print!("}}");
}

// takes a line from the file and returns a tuple of String, Temperature
fn split_lines(line: &str) -> (&str, f64) {
    let (station, temp) = line.split_once(';').unwrap();

    let temp: f64 = temp.parse().unwrap();

    return (station, temp);
}
