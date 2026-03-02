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
    // Store the key as a Vec<u8> instead for speed.
    let mut map: HashMap<Vec<u8>, Info> = HashMap::with_capacity(10_000);

    // read the data in the file line by line
    let file = File::open("../data/measurements.txt").unwrap();
    // in the future, allocate a buffer with_capacity = to the bytes in the file
    let reader = BufReader::new(file);

    for line in reader.split(b'\n') {
        let line = line.unwrap();
        let (station, temp) = split_lines(&line);

        // only allcate on new entries

        let stats = match map.get_mut(&station) {
            Some(stats) => stats,
            None => map.entry(station.to_vec()).or_default(),
        };

        stats.min = stats.min.min(temp);
        stats.max = stats.max.max(temp);
        stats.count += 1;
        stats.total += temp;
    }
    let mut sorted: Vec<(Vec<u8>, Info)> = map.into_iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));

    print!("{{");
    for (station, info) in sorted {
        let mean = info.total / info.count as f64;
        let station = std::str::from_utf8(&station).unwrap();

        print!("{station} = {}/{:.1}/{}", info.min, mean, info.max);

        print!(", ")
    }

    print!("}}");
}

fn split_lines(line: &[u8]) -> (Vec<u8>, f64) {
    // splitn(2) on ';' gives [station, temp] in forward order
    let mut fields = line.splitn(2, |c| *c == b';');
    let station = fields.next().unwrap();
    let temp = fields.next().unwrap();
    let temp: f64 = unsafe { std::str::from_utf8_unchecked(temp) }
        .parse()
        .unwrap();

    (station.to_vec(), temp)
}
