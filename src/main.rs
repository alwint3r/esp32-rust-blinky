use embedded_hal::{digital::v2::OutputPin, prelude::_embedded_hal_blocking_delay_DelayMs};
use esp_idf_hal::{prelude::Peripherals, delay};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    // let netif_stack = Arc::new(EspNetif)
    println!("Hello world!");

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let mut io5 = pins.gpio5.into_output().unwrap();
    let mut delay = delay::Ets;

    let delay_time: u32 = 1000;
    
    loop {
        io5.set_state(embedded_hal::digital::v2::PinState::Low).unwrap();
        delay.delay_ms(delay_time);
        io5.set_state(embedded_hal::digital::v2::PinState::High).unwrap();
        delay.delay_ms(delay_time);
    }
}
