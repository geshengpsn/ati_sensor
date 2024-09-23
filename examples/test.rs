use std::{thread::sleep, time::Duration};

use ati_sensor::AtiNano25;

fn main() {
    let mut ati = AtiNano25::new("192.168.1.1:49151");
    for _ in 0..10 {
        let force = ati.read_force();
        println!("{:?}", force);
        sleep(Duration::from_secs_f64(0.1));
    }
    ati.set_zero();
    loop {
        let force = ati.read_force();
        println!("{:?}", force);
        sleep(Duration::from_secs_f64(0.1));
    }
}