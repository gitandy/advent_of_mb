#![no_std]
#![no_main]

use defmt::{info, debug};
use embassy_executor::Spawner;
use embassy_nrf::{gpio::Pin,
                  saadc,
                  bind_interrupts};
use embassy_time::{Duration, Timer};
use microbit_bsp::{
    Microbit,
    embassy_nrf::{
        pwm::SimplePwm,
        saadc::{Saadc, Config, ChannelConfig}},
    speaker::{NamedPitch, Note, PwmSpeaker, Pitch},
};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    SAADC => saadc::InterruptHandler;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    info!("Initializing...");
    let board = Microbit::default();

    let mut speaker = PwmSpeaker::new(SimplePwm::new_1ch(board.pwm0, board.speaker));
    let adc_cfg = Config::default();
    let ch_cfg = ChannelConfig::single_ended(board.p0);
    let mut adc = Saadc::new(board.saadc, Irqs, adc_cfg, [ch_cfg]);

    info!("Running...");
    loop {
        let mut buf:[i16;1]= [0];
        adc.sample(&mut buf).await;
        let freq = (buf[0]/3) as u32 + 200;  // ~ 200 - 1500 Hz
        speaker.start_note(Pitch::Frequency(freq));
        Timer::after_millis(500).await;
    }
}
