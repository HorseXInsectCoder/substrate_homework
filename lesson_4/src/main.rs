
// 1. ----------------------------------------------------------------

use std::f64::consts::PI;

const RED_TIME: u32 = 20;
const GREEN_TIME: u32 = 10;
const YELLOW_TIME: u32 = 5;

#[derive(PartialEq)]
enum TrafficLight {
    Red,
    Green,
    Yellow,
}

trait Traffic {
    fn time(&self) -> u32;
}

impl Traffic for TrafficLight {
    fn time(&self) -> u32 {
        match self {
            TrafficLight::Red => RED_TIME,
            TrafficLight::Green => GREEN_TIME,
            TrafficLight::Yellow => YELLOW_TIME,
        }
    }
}

// 2. ----------------------------------------------------------------
fn sum(list: &[u32]) -> Option<u32> {
    let mut sum = 0;
    for item in list {
        if item >= &u32::MAX {
            return None
        }
        sum += item
    }
    Some(sum)
}

// 3. ----------------------------------------------------------------
trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64
}

struct Triangle {
    length: f64,
    height: f64,
}

struct Rectangle {
    length: f64,
    height: f64,
}

fn print_area<T: Area>(t: &T) -> f64 {
    t.area()
}

impl Area for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * PI
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        self.height * self.length / 2.0
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.height * self.length
    }
}

fn main() {

    // 1. --------------------------------------
    let red = TrafficLight::Red;
    let green = TrafficLight::Green;
    let yellow = TrafficLight::Yellow;

    assert_eq!(Traffic::time(&red), 20);
    assert_eq!(Traffic::time(&green), 10);
    assert_eq!(Traffic::time(&yellow), 5);

    // 2. --------------------------------------
    let list = vec![1, u32::MAX];
    assert_eq!(sum(&list), None);

    // 3. --------------------------------------
    let circle = Circle {
        radius: 6.0,
    };
    let triangle = Triangle {
        length: 3.0,
        height: 4.0
    };
    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };

    println!("{:?}", print_area(&circle));
    println!("{:?}", print_area(&triangle));
    println!("{:?}", print_area(&rectangle));
}
