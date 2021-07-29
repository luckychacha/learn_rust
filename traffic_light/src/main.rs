
enum TrafficLight {
    Red,
    Green,
    Yellow
}

pub trait TrafficLightTime {
    fn get_light_time(&self) -> u32;
}

impl TrafficLightTime for TrafficLight {
    fn get_light_time(&self) -> u32 {
        match &self {
            TrafficLight::Red => {
                1
            }
            TrafficLight::Green => {
                2
            }
            TrafficLight::Yellow => {
                3
            }
        }
    }
}

fn main() {
    println!("Hello, world!");

    println!("{:?}", TrafficLight::Red.get_light_time());
    println!("{:?}", TrafficLight::Green.get_light_time());
    println!("{:?}", TrafficLight::Yellow.get_light_time());
}
