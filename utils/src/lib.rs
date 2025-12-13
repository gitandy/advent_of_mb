#![no_std]
// If RustRover claims missing test due to no_std got to
// File -> Settings... -> Rust -> External Linters -> Additional arguments: --target thumbv7m-none-eabihf

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use nrf52833_hal::Timer;
use nrf52833_hal::gpio::p0::P0_00;
use nrf52833_hal::gpio::{Output, PushPull};
use nrf52833_hal::pac::TIMER0;

pub fn play_tone(
    timer_f: &mut Timer<TIMER0>,
    speaker: &mut P0_00<Output<PushPull>>,
    freq: u32,
    duration: u32,
) {
    // Todo: fix duration
    let period = 1000 / freq;
    for _ in 0..duration / period {
        timer_f.delay_ms(period / 2);
        speaker.set_high().unwrap();
        timer_f.delay_ms(period / 2);
        speaker.set_low().unwrap();
    }
}
