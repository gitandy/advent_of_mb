#![no_std]
#![no_main]

use core::ptr::write_volatile;
use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Initializing...");

    // Uses PIN_CNF registers instead of DIR
    const PCNF_ROW1_ADDR: *mut u32 = 0x5000_0754 as *mut u32;
    const PCNF_COL1_ADDR: *mut u32 = 0x5000_0770 as *mut u32;
    const DIR_OUTPUT_POS: u32 = 0;
    const DRIVE_LED: u32 = 1 << DIR_OUTPUT_POS;

    unsafe {
        write_volatile(PCNF_ROW1_ADDR, DRIVE_LED);
        write_volatile(PCNF_COL1_ADDR, DRIVE_LED);
    }

    const OUT_ADDR: *mut u32 = 0x5000_0504 as *mut u32;
    const ROW1_POS: u32 = 21;
    let mut on: bool = false;

    loop {
        unsafe {
            write_volatile(OUT_ADDR, (on as u32) << ROW1_POS);
        }

        for _ in 0..500_000 {
            nop();
        }

        on = !on;
        rprintln!("is on: {}", on);
    }
}
