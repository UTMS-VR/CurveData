use std::env;
use std::fs::File;
use std::io::BufReader;
use serde_json::Value;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let f = File::open(filename).expect("file not found");
    let reader = BufReader::new(f);
    let deserialized: Value = serde_json::from_reader(reader).unwrap();

    let curve = twice(&value_as_curve(&deserialized));
    curve_to_obj(&curve);
}

#[derive(Debug)]
struct Point(f64, f64, f64);

#[derive(Debug)]
struct Curve {
    points: Vec<Point>,
    closed: bool,
}

fn value_as_point(v: &Value) -> Point {
    Point(v["x"].as_f64().unwrap(), v["y"].as_f64().unwrap(), v["z"].as_f64().unwrap())
}

fn value_as_curve(value: &Value) -> Curve {
    let value = &value["target"][0];
    let list = value["points"].as_array().unwrap();
    let list: Vec<Point> = list.iter().map(|v| value_as_point(v)).collect();
    let b = value["closed"].as_bool().unwrap();
    Curve { points: list, closed: b }
}

fn curve_to_obj(curve: &Curve) {
    let len = curve.points.len();
    for point in curve.points.iter() {
        println!("v {} {} {}", point.0, point.1, point.2);
    }
    for i in 0..len {
        println!("l {} {}", i + 1, (i + 1) % len + 1)
    }
}

fn twice(curve: &Curve) -> Curve {
    match curve.closed {
        true => {
            let len = curve.points.len();
            let mut newpoints: Vec<Point> = Vec::new();
            for i in 0..len {
                newpoints.push(copy_point(&curve.points[i]));
                let j = (i + 1) % len;
                newpoints.push(average_point(&curve.points[i],  &curve.points[j]));
            };
            Curve { points: newpoints, closed: true}
        },
        false => panic!(),
    }
}

fn average_point(point1: &Point, point2: &Point) -> Point {
    Point((point1.0 + point2.0) / 2.0, (point1.1 + point2.1) / 2.0, (point1.2 + point2.2) / 2.0)
}

fn copy_point(point: &Point) -> Point {
    Point(point.0, point.1, point.2)
}