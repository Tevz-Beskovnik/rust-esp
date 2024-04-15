use anyhow::Result;
use esp_idf_svc::hal::{delay::Delay, gpio::PinDriver, peripherals::Peripherals};
use rgb::RGB8;
use rust_esp::rgb_led::WS2812RMT;

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    let peripherals = Peripherals::take().expect("Failed to get peripherals");

    let mut rgb_led = WS2812RMT::new(peripherals.pins.gpio8, peripherals.rmt.channel0)?;

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");
    log::info!("Hello, world, again ahaha!");

    let delay: Delay = Default::default();

    loop {
        rgb_led.set_pixel(RGB8::new(100, 100, 100))?;
        log::info!("Set high!");
        delay.delay_us(1000000);
        rgb_led.set_pixel(RGB8::new(0, 0, 0))?;
        log::info!("Set low!");
        delay.delay_us(1000000);
    }
}
