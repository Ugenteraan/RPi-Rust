use std::error::Error;
use std::thread;
use std::time::Duration;

#[cfg(target_arch = "aarch64")]
use rppal::gpio::Gpio;

const GPIO_LED: u8 = 4;

fn main() -> Result<(), Box<dyn Error>> {
    let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();

    loop {
        pin.toggle();
        thread::sleep(Duration::from_millis(500));
    }
}
