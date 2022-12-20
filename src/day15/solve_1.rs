use regex::Regex;
use std::collections::HashSet;
use std::fs;

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

fn count_beacons_on_a_row(tgt_row: i64, x_min: i64, x_max: i64, sensors: &Vec<Sensor>) -> i32 {
    let mut count = 0;
    for sensor in sensors {
        if sensor.beacon.1 == tgt_row && sensor.beacon.0 >= x_min && sensor.beacon.0 <= x_max {
            count += 1;
        }
    }
    return count;
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

    let mut min_x = &sensor_array[0].loc.0 - sensor_array[0].radius;
    let mut max_x = &sensor_array[0].loc.0 + sensor_array[0].radius;

    for sensor in &sensor_array {
        min_x = min_x.min(sensor.loc.0 - sensor.radius);
        max_x = max_x.max(sensor.loc.0 + sensor.radius);
    }

    const TARGET_ROW: i64 = 2000000;
    let mut nr_impossible = 0;
    println!("Checking line {} from {} to {}", TARGET_ROW, min_x, max_x);

    let mut count_beacons = 0;
    for beacon in beacon_set {
        if beacon.1 == TARGET_ROW && beacon.0 >= min_x && beacon.0 <= max_x {
            count_beacons += 1;
        }
    }

    for x in min_x..=max_x {
        // we count the number of points where the emergency beacon cannot be,
        // i.e. points which are closer to one sensor from the corresponding radius
        if is_close_to_sensor((x, TARGET_ROW), &sensor_array) {
            // println!("   Column {} is close to a sensor!", x);
            nr_impossible += 1;
        }
    }

    println!("  * Number of points close to sensors: {}", nr_impossible);
    println!("  * Number of beacons on the row: {}", count_beacons);
    println!(
        "The answer to question 1 is {}",
        nr_impossible - count_beacons
    );
}
