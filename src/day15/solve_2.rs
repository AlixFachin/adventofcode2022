use regex::Regex;
use std::collections::HashSet;
use std::{fs, vec};

const FILEPATH: &str = "src/day15/input.txt";

// returns manhattan distance
fn dist(a: (i64, i64), b: (i64, i64)) -> i64 {
    return a.0.max(b.0) - a.0.min(b.0) + a.1.max(b.1) - a.1.min(b.1);
}

#[test]
fn test_dist() {
    assert_eq!(dist((0, 0), (5, 5)), 10);
    assert_eq!(dist((0, 0), (-5, 5)), 10);
    assert_eq!(dist((0, 0), (5, -5)), 10);
    assert_eq!(dist((0, 0), (-5, -5)), 10);

    assert_eq!(dist((5, 5), (0, 0)), 10);
    assert_eq!(dist((-5, 5), (0, 0)), 10);
    assert_eq!(dist((5, -5), (0, 0)), 10);
    assert_eq!(dist((-5, -5), (0, 0)), 10);

    assert_eq!(dist((0, 0), (0, 0)), 0);
}

struct Sensor {
    loc: (i64, i64),
    beacon: (i64, i64),
    radius: i64,
}

fn is_close_to_sensor(tgt: (i64, i64), sensors: &Vec<Sensor>) -> bool {
    let mut i = 0;
    for sensor in sensors {
        let dist_sensor = dist(tgt, sensor.loc);
        if dist_sensor <= sensor.radius {
            // println!("      Closest point from {:?} is {:?}", tgt, sensor.loc);
            return true;
        }
        i += 1;
    }
    return false;
}

#[test]
fn test_close_to_sensor() {
    let test_sensor_array = vec![Sensor {
        loc: (10, 10),
        beacon: (15, 15),
        radius: 10,
    }];

    assert!(is_close_to_sensor((5, 5), &test_sensor_array));
    assert!(is_close_to_sensor((6, 6), &test_sensor_array));
    assert!(is_close_to_sensor((10, 10), &test_sensor_array));
    assert!(is_close_to_sensor((15, 15), &test_sensor_array));

    assert!(!is_close_to_sensor((4, 5), &test_sensor_array));
    assert!(!is_close_to_sensor((5, 4), &test_sensor_array));
    assert!(!is_close_to_sensor((16, 15), &test_sensor_array));
    assert!(!is_close_to_sensor((15, 16), &test_sensor_array));
    assert!(!is_close_to_sensor((17, 17), &test_sensor_array));
}

pub fn solve() {
    let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");
    let line_array: Vec<&str> = contents.split("\n").collect();

    let re_sensor: Regex = Regex::new(r"Sensor at x=(?P<sx>\-?\d+), y=(?P<sy>\-?\d+): closest beacon is at x=(?P<bx>\-?\d+), y=(?P<by>\-?\d+)").unwrap();

    let mut sensor_array: Vec<Sensor> = Vec::new();

    let mut beacon_set = HashSet::new();

    for line in line_array {
        println!("Checking line: {}", line);
        let matches = re_sensor.captures(line).unwrap();

        let sx = matches["sx"].parse::<i64>().unwrap();
        let sy = matches["sy"].parse::<i64>().unwrap();
        let bx = matches["bx"].parse::<i64>().unwrap();
        let by = matches["by"].parse::<i64>().unwrap();

        sensor_array.push(Sensor {
            loc: (sx, sy),
            beacon: (bx, by),
            radius: dist((sx, sy), (bx, by)),
        });

        beacon_set.insert((bx, by));
    }

    for y in 0..4_000_000 {
        let mut vec_intervals: Vec<(i64, i64)> = Vec::new();
        for sensor in &sensor_array {
            if i64::abs(sensor.loc.1 - y) > sensor.radius {
            } else {
                let d = i64::abs(sensor.loc.1 - y);
                let x_min = sensor.loc.0 - sensor.radius + d;
                let x_max = sensor.loc.0 + sensor.radius - d;
                vec_intervals.push((x_min, x_max));
            }
        }

        vec_intervals.sort_by(|a, b| a.0.cmp(&b.0));

        let mut x: i64 = 0;
        for interval in vec_intervals {
            if x >= interval.0 && x >= interval.1 {
                // the whole interval is behind us, continue
                continue;
            }
            if x >= interval.0 && x <= interval.1 {
                // we skip to the end of the interval
                x = interval.1;
            } else {
                println!(
                    "!!!!!!!!!!!!!!!!!!!!!!!!!!!Interval outside of range for row {}: between {} and {}",
                    y, x, interval.0
                );
                println!("Solution for question 2 is: {}", (x + 1) * 4_000_000 + y);
                panic!("I think we found what we were looking for?");
            }
        }

        if x > 4_000_000 {
            println!("No acceptable point for y={}", y);
        }
    }
}
