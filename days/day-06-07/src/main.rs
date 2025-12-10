#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin, OutputPin};
use nrf52833_hal::{gpio, pac, timer};
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Initializing...");
    let peripherals = pac::Peripherals::take().unwrap();
    let p0 = gpio::p0::Parts::new(peripherals.P0);
    let mut timer_f = timer::Timer::new(peripherals.TIMER0);

    // Ring 0
    let mut hot_wire = p0.p0_02.into_pullup_input();
    // Internal speaker
    let mut speaker = p0.p0_00.into_push_pull_output(gpio::Level::Low);

    let freq = 420;
    let period = 1000u32 / freq;

    rprintln!("Running...");
    loop {
        if hot_wire.is_low().unwrap() {
            for _ in 0..1000 / period {
                timer_f.delay_ms(period / 2);
                speaker.set_high().unwrap();
                timer_f.delay_ms(period / 2);
                speaker.set_low().unwrap();
            }
        }
    }
}
