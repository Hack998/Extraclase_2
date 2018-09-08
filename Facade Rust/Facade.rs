struct Engine;
struct Headlights;
struct Car;

impl Engine {
    fn Start(&self){}
}
impl Headlights {
    fn TurnOn(&self){}
}
impl Car {
    fn TurnIgnitionKeyOn(&self) {
        let e = Engine{};
        let h = Headlights{};
        println!("The car is on");
    }
}

fn main() {
    let c = Car{};
    c.TurnIgnitionKeyOn();
}﻿
