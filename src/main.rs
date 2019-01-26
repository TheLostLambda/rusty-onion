extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::thread::{spawn, sleep};
use std::time::Duration;

fn main() {
    let red = Pin::new(0);
    let green = Pin::new(18);
    let blue = Pin::new(2);
    let buzzer = Pin::new(19);
    let left_button = Pin::new(3);

    batch_export(vec![red, green, blue, buzzer, left_button]);

    left_button.set_direction(Direction::In).expect("Failed to set mode to input");

    let red_copy = red.clone();
    spawn(move || { blink(red_copy,1000,2); });
    sleep(Duration::from_millis(250));
    blink(blue,1000,2);

    let mut last = left_button.get_value().unwrap();
    let mut hold = 0;
    loop {
        let left = left_button.get_value().unwrap();
        if hold > 500 {
            println!("Program exited!");
            green.set_value(0).expect("Failed to set pin value");
            break;
        }
        if last == 1 && left == 0 {
            println!("Button was pressed!");
            let buzzer_copy = buzzer.clone();
            spawn(move || { blink(buzzer_copy,250,500);
                            blink(buzzer_copy,250,250);});
        }
        if last == 1 && left == 1 {
            hold += 1;
            green.set_value(1).expect("Failed to set pin value");
        } else {
            hold = 0;
            green.set_value(0).expect("Failed to set pin value");
        }
        last = left;
        sleep(Duration::from_millis(1));
    }
}

fn blink(pin: Pin, dur_ms: u64, freq: u64) {
    let per_ms = 1_000.0 / (freq as f64);
    let per_ms = per_ms.ceil() as u64;
    for _ in 1..=(dur_ms / per_ms) {
        pin.set_value(1).expect("Failed to set pin value");
        sleep(Duration::from_millis(per_ms/2));
        pin.set_value(0).expect("Failed to set pin value");
        sleep(Duration::from_millis(per_ms/2));
    }
}

fn batch_export(pins: Vec<Pin>) {
    for pin in pins {
        if !pin.is_exported() {
            pin.export().expect(
                &format!("Failed to export GPIO pin {}", pin.get_pin_num()));
        }
    }
}

/*macro_rules! launch {
    => { val_copy = $val.clone
        spawn(move || { blink(val_copy,1000,2); });}*/
