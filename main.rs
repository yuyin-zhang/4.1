
fn main() {
    let redlight = TrafficLight::Red;
    println!("redlight is: {}", redlight.redtime());
    let greenlight = TrafficLight::Green;
    println!("greenlight is: {}", greenlight.greentime());
    let yellowlight = TrafficLight::Yellow;
    println!("yellowlight is: {}", yellowlight.yellowtime());
    
}

enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl TrafficLight {
    fn redtime(&self) -> u8 {
        60
    }
    fn greentime(&self) -> u8 {
        70
    }
    fn yellowtime(&self) -> u8 {
        20
    }
}