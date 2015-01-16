extern crate time;
use std::rand;

static EPOCH:f64 = 946702800.0;
static TIMESTAMP_SHIFT:usize = 23;
static RANDOM_MAX_VALUE:u32 = 4194303;

#[derive(Copy)]
pub struct Id {
    pub timestamp: f64,
    pub random_bits: u32,
}

#[allow(unstable)]
pub fn new() -> u64 {
    let now = time_as_float(time::get_time());
    let timestamp = ((now - EPOCH) * 1000.0) as u64;
    let random_bits = rand::random::<u32>() % RANDOM_MAX_VALUE;

    (timestamp << TIMESTAMP_SHIFT) | (random_bits as u64)
}

pub fn parse(id:u64) -> Id {
    let random_bits = (id as u32) & RANDOM_MAX_VALUE;
    let mut timestamp = (id >> TIMESTAMP_SHIFT) as f64;
    timestamp /= 1000.0;
    timestamp += EPOCH;

    Id { timestamp: timestamp, random_bits: random_bits }
}

fn time_as_float(spec:time::Timespec) -> f64 {
    let mut nsec = spec.nsec as f64;
    while nsec > 1.0 {
        nsec /= 10.0;
    }

    (spec.sec as f64) + nsec
}

#[test]
fn test_time_as_float() {
    let input = time::Timespec { sec: 12, nsec: 34};
    let output = time_as_float(input);

    assert_eq!(output, 12.34);
}
