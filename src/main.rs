use core::f64;
use hashbrown::hash_map::Entry;
use hashbrown::HashMap;
use memchr::memchr;
use memmap2::Mmap;
use rayon::prelude::*;
use std::fs::File;

struct Info {
    min: f64,
    count: usize,
    total: f64,
    max: f64,
}

fn main() {
    let file = File::open("../data/measurements.txt").unwrap();
    let mmap = unsafe { Mmap::map(&file) }.unwrap();
    mmap.advise(memmap2::Advice::Sequential).unwrap();

    let num_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);

    let chunks = split_at_newlines(&mmap, num_threads);

    let maps: Vec<HashMap<&[u8], Info>> = chunks
        .par_iter()
        .map(|chunk| process_chunk(chunk))
        .collect();

    let merged = merge(maps);

    let mut sorted: Vec<(&&[u8], &Info)> = merged.iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(b.0));

    print!("{{");
    for (station, info) in sorted {
        let mean = info.total / info.count as f64;
        let station = std::str::from_utf8(station).unwrap();

        print!("{station} = {}/{:.1}/{}", info.min, mean, info.max);

        print!(", ")
    }

    print!("}}");
}

// zero copy &[u8] slice better than Vec
// TODO: Optimize using memchr instead of iterating the line
fn split_lines(line: &[u8]) -> (&[u8], f64) {
    let sep = memchr(b';', line).unwrap();
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

fn split_at_newlines<'a>(data: &'a [u8], num_chunks: usize) -> Vec<&'a [u8]> {
    let chunk_size = data.len() / num_chunks;
    let mut chunks = Vec::with_capacity(num_chunks);
    let mut start = 0;

    for i in 1..num_chunks {
        let mut end = i * chunk_size;

        while end < data.len() && data[end] != b'\n' {
            end += 1;
        }

        end += 1;
        chunks.push(&data[start..end]);
        start = end;
    }

    chunks.push(&data[start..]);
    chunks
}

fn process_chunk(chunk: &[u8]) -> HashMap<&[u8], Info> {
    let mut map: HashMap<&[u8], Info> = HashMap::with_capacity(5_000);

    for line in chunk.split(|c| *c == b'\n') {
        if line.is_empty() {
            break;
        }

        let (station, temp) = split_lines(line);

        // if the entry exists, put new info
        match map.entry(station) {
            Entry::Occupied(mut e) => {
                let s = e.get_mut();
                s.min = s.min.min(temp);
                s.max = s.max.max(temp);
                s.count += 1;
                s.total += temp;
            }

            // if entry doesnt exist, put default
            Entry::Vacant(e) => {
                e.insert(Info {
                    min: temp,
                    max: temp,
                    count: 1,
                    total: temp,
                });
            }
        }
    }

    map
}

fn merge<'a>(maps: Vec<HashMap<&'a [u8], Info>>) -> HashMap<&'a [u8], Info> {
    let mut final_map: HashMap<&[u8], Info> = HashMap::with_capacity(10_000);

    for map in maps {
        for (station, info) in map {
            match final_map.entry(station) {
                Entry::Occupied(mut e) => {
                    let s = e.get_mut();
                    s.min = s.min.min(info.min);
                    s.max = s.max.max(info.max);
                    s.count += info.count;
                    s.total += info.total;
                }
                Entry::Vacant(e) => {
                    e.insert(info);
                }
            }
        }
    }

    final_map
}
