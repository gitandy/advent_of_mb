#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin, OutputPin};
use microbit::hal::{gpio, timer};
use microbit::adc::{Adc, AdcConfig};
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

    // let mut btn_a = board.buttons.button_a.into_pullup_input();
    let mut speaker = board.speaker_pin.into_push_pull_output(gpio::Level::Low);
    // Touch field logo
    // let mut touch_face = board.pins.p1_04.into_floating_input();
    let adc_cfg = AdcConfig::default();
    let mut adc =  Adc::new(board.ADC, adc_cfg);
    let mut touch = board.edge.e00;

    rprintln!("Running...");
    loop {
        // let rnd_nmbr = rng.random_range(0..15);
        let val_raw = adc.read_channel(&mut touch).unwrap();
        let val = (val_raw - 8380) as f32 / 8000f32;
        rprintln!("{} {} {}", val_raw, val_raw - 8380, val);
        timer_f.delay_ms(2000);


        //     if btn_a.is_low().unwrap() {
        //         play_tone(&mut timer_f, &mut speaker, 250, 500);
        //         play_tone(&mut timer_f, &mut speaker, 400, 1000);
        //     } else {
        //         play_tone(&mut timer_f, &mut speaker, 400, 1000);
        //         play_tone(&mut timer_f, &mut speaker, 250, 2000);
        //     }
        // } else if btn_a.is_low().unwrap() {
        //     play_tone(&mut timer_f, &mut speaker, 400, 1000);
        //     play_tone(&mut timer_f, &mut speaker, 250, 2000);
        // }
    }
}
