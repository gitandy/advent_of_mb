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
    
    let mut timer_f = timer::Timer::new(board.TIMER0);
    let mut speaker = board.speaker_pin.into_push_pull_output(gpio::Level::Low);

    let adc_cfg = AdcConfig::default();
    let mut adc =  Adc::new(board.ADC, adc_cfg);
    let mut touch = board.edge.e00;

    rprintln!("Running...");
    loop {
        let val_raw = adc.read_channel(&mut touch).unwrap();
        let mut val = (val_raw - 8380) as f32 / 8000f32;
        val = if val > 1f32 {1f32} else if val < 0f32 {0f32} else {val};
        let freq = ((1f32-val)*1000f32) as u32 + 200u32;
        // rprintln!("{}", freq);

        play_tone(&mut timer_f, &mut speaker, freq, 500);
    }
}
