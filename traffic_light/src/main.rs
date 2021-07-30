use std::time::Duration;

enum TrafficLight {
    Red,
    Green,
    Yellow
}

pub trait TrafficLightDuration {
    fn get_duration(&self) -> Duration;
}

impl TrafficLightDuration for TrafficLight {
    fn get_duration(&self) -> Duration {
        match &self {
            TrafficLight::Red => {
                Duration::from_secs(1)
            }
            TrafficLight::Green => {
                Duration::from_secs(2)
            }
            TrafficLight::Yellow => {
                Duration::from_secs(3)
            }
        }
    }
}

fn main() {
    println!("Hello, world!");

    println!("{:?}", TrafficLight::Red.get_duration());
    println!("{:?}", TrafficLight::Green.get_duration());
    println!("{:?}", TrafficLight::Yellow.get_duration());
}
