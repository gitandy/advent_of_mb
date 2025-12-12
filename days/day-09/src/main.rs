#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin, OutputPin};
use microbit::hal::{gpio, timer};
use panic_halt as _;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rtt_target::{rprintln, rtt_init_print};
use utils::play_tone;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Initializing...");
    let board = microbit::Board::take().unwrap();

    let mut rng = SmallRng::seed_from_u64(545451234567u64);
    let mut timer_f = timer::Timer::new(board.TIMER0);

    let mut btn_a = board.buttons.button_a.into_pullup_input();
    let mut speaker = board.speaker_pin.into_push_pull_output(gpio::Level::Low);

    // Provide + to the first row of LEDs
    let _row1 = board.display_pins.row1.into_push_pull_output(gpio::Level::High);
    // Set initial + so LEDs are off
    let mut leds = (
        board.display_pins.col1.into_push_pull_output(gpio::Level::High),
        board.display_pins.col2.into_push_pull_output(gpio::Level::High),
        board.display_pins.col3.into_push_pull_output(gpio::Level::High),
        board.display_pins.col4.into_push_pull_output(gpio::Level::High),
    );

    rprintln!("Running...");
    loop {
        let rnd_nmbr = rng.random_range(0..15);

        if rnd_nmbr & 8 == 8 {
            leds.0.set_low().unwrap();
        } else {
            leds.0.set_high().unwrap();
        }
        if rnd_nmbr & 4 == 4 {
            leds.1.set_low().unwrap();
        } else {
            leds.1.set_high().unwrap();
        }
        if rnd_nmbr & 2 == 2 {
            leds.2.set_low().unwrap();
        } else {
            leds.2.set_high().unwrap();
        }
        if rnd_nmbr & 1 == 1 {
            leds.3.set_low().unwrap();
        } else {
            leds.3.set_high().unwrap();
        }
        timer_f.delay_ms(400);

        if rnd_nmbr == 12 {
            if btn_a.is_low().unwrap() {
                play_tone(&mut timer_f, &mut speaker, 250, 500);
                play_tone(&mut timer_f, &mut speaker, 400, 1000);
            } else {
                play_tone(&mut timer_f, &mut speaker, 400, 1000);
                play_tone(&mut timer_f, &mut speaker, 250, 2000);
            }
        } else if btn_a.is_low().unwrap() {
            play_tone(&mut timer_f, &mut speaker, 400, 1000);
            play_tone(&mut timer_f, &mut speaker, 250, 2000);
        }
    }
}
