use std::collections::HashSet;

use regex::Regex;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Sensor {
    pub pos: Pos,
    pub max_distance: isize,
}

pub type Beacon = Pos;

const LINE_REGEX: &str = r"^.*x=(-?\d*), y=(-?\d*):.*x=(-?\d*), y=(-?\d*)$";

pub fn parse_input(input: &str) -> (HashSet<Sensor>, HashSet<Beacon>) {
    let mut sensors = HashSet::new();
    let mut beacons = HashSet::new();

    for line in input.lines() {
        let re = Regex::new(LINE_REGEX).unwrap();
        let matches = re.captures(line).unwrap();

        let mut sensor = Sensor {
            pos: Pos {
                x: matches[1].parse().unwrap(),
                y: matches[2].parse().unwrap(),
            },
            max_distance: 0,
        };

        let beacon = Beacon {
            x: matches[3].parse().unwrap(),
            y: matches[4].parse().unwrap(),
        };

        let max_distance = sensor.pos.x.abs_diff(beacon.x) + sensor.pos.y.abs_diff(beacon.y);
        sensor.max_distance = max_distance as isize;

        sensors.insert(sensor);
        beacons.insert(beacon);
    }

    (sensors, beacons)
}

impl Sensor {
    pub fn is_pos_in_range(&self, pos: &Pos) -> bool {
        self.distance_from(pos) <= self.max_distance
    }

    pub fn distance_from(&self, pos: &Pos) -> isize {
        (self.pos.x.abs_diff(pos.x) + self.pos.y.abs_diff(pos.y))
            .try_into()
            .unwrap()
    }
}
