#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin, OutputPin};
use nrf52833_hal::gpio::p0::P0_00;
use nrf52833_hal::gpio::{Output, PushPull};
use nrf52833_hal::pac::TIMER0;
use nrf52833_hal::{Timer, gpio, pac, timer};

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Initializing...");
    let peripherals = pac::Peripherals::take().unwrap();
    let p0 = gpio::p0::Parts::new(peripherals.P0);
    let mut timer_f = timer::Timer::new(peripherals.TIMER0);

    // Button A
    let mut btn_a = p0.p0_14.into_pullup_input();
    // Button B
    let mut btn_b = p0.p0_23.into_pullup_input();
    // Internal speaker
    let mut speaker = p0.p0_00.into_push_pull_output(gpio::Level::Low);

    rprintln!("Running...");
    loop {
        if btn_a.is_low().unwrap() {
            play_tone(&mut timer_f, &mut speaker, 250, 500);
            timer_f.delay_ms(100);
            play_tone(&mut timer_f, &mut speaker, 400, 1000);
        } else if btn_b.is_low().unwrap() {
            play_tone(&mut timer_f, &mut speaker, 400, 1000);
            play_tone(&mut timer_f, &mut speaker, 250, 2000);
        }
    }
}

fn play_tone(
    timer_f: &mut Timer<TIMER0>,
    speaker: &mut P0_00<Output<PushPull>>,
    freq: u32,
    duration: u32,
) {
    let period = 1000 / freq;
    for _ in 0..duration / period {
        timer_f.delay_ms(period / 2);
        speaker.set_high().unwrap();
        timer_f.delay_ms(period / 2);
        speaker.set_low().unwrap();
    }
}
