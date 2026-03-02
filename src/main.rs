use core::f64;
use memmap2::Mmap;
use std::collections::HashMap;
use std::fs::File;

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
    let mut map: HashMap<&[u8], Info> = HashMap::with_capacity(10_000);

    // read the data in the file line by line
    let file = File::open("../data/measurements.txt").unwrap();
    // in the future, allocate a buffer with_capacity = to the bytes in the file

    let mmap = unsafe { Mmap::map(&file) }.unwrap();

    for line in mmap.split(|c| *c == b'\n') {
        if line.is_empty() {
            break;
        }

        let (station, temp) = split_lines(line);

        match map.entry(station) {
            std::collections::hash_map::Entry::Occupied(mut e) => {
                let s = e.get_mut();
                s.min = s.min.min(temp);
                s.max = s.max.max(temp);
                s.count += 1;
                s.total += temp;
            }

            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(Info {
                    min: temp,
                    max: temp,
                    count: 1,
                    total: temp,
                });
            }
        }
    }

    let mut sorted: Vec<(&&[u8], &Info)> = map.iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(b.0));

    print!("{{");
    for (station, info) in sorted {
        let mean = info.total / info.count as f64;
        let station = std::str::from_utf8(&station).unwrap();

        print!("{station} = {}/{:.1}/{}", info.min, mean, info.max);

        print!(", ")
    }

    print!("}}");
}

// zero copy &[u8] slice better than Vec
fn split_lines(line: &[u8]) -> (&[u8], f64) {
    let sep = line.iter().position(|&c| c == b';').unwrap();
    let station = &line[..sep];
    let temp = parse_temp(&line[sep + 1..]);
    (station, temp)
}

// parse out the temp from slice of bytes.. this is some bullshit
fn parse_temp(bytes: &[u8]) -> f64 {
    let (neg, bytes) = if bytes[0] == b'-' {
        (true, &bytes[1..])
    } else {
        (false, bytes)
    };
    let val: i32 = match bytes {
        [a, b'.', c] => (*a - b'0') as i32 * 10 + (*c - b'0') as i32,
        [a, b, b'.', c] => (*a - b'0') as i32 * 100 + (*b - b'0') as i32 * 10 + (*c - b'0') as i32,
        _ => panic!("unexpected temp format: {:?}", std::str::from_utf8(bytes)),
    };
    let val = if neg { -val } else { val };
    val as f64 / 10.0
}
