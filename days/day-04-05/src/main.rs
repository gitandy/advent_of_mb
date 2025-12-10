#![no_std]
#![no_main]

use core::ptr::{read_volatile, write_volatile};
use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Initializing...");

    let mut rng = SmallRng::seed_from_u64(545451234567u64);

    // Row 1 P0.21
    const ROW1_ADDR: *mut u32 = 0x5000_0754 as *mut u32;
    // Col 1 P0.28
    const COL1_ADDR: *mut u32 = 0x5000_0770 as *mut u32;
    const COL2_ADDR: *mut u32 = 0x5000_072c as *mut u32;
    const COL3_ADDR: *mut u32 = 0x5000_077c as *mut u32;
    const DIR_OUTPUT_POS: u32 = 0;
    const DRIVE_LED: u32 = 1 << DIR_OUTPUT_POS;

    // Btn A P0.14, unnecessary: should be 0 be default
    const BTN_A_ADDR: *mut u32 = 0x5000_0738 as *mut u32;
    const DRIVE_INPUT: u32 = 0 << DIR_OUTPUT_POS;

    const OUT_ADDR: *mut u32 = 0x5000_0504 as *mut u32;
    const ROW1_POS: u32 = 21;
    const COL1_POS: u32 = 28;
    const COL2_POS: u32 = 11;
    const COL3_POS: u32 = 31;
    // let mut on:bool = false;

    // Drive COLs high to deactivate LEDs and ROW to high to provide + on LEDS
    let out: u32 = 1 << COL1_POS | 1 << COL2_POS | 1 << COL3_POS | 1 << ROW1_POS;

    unsafe {
        write_volatile(ROW1_ADDR, DRIVE_LED);
        write_volatile(COL1_ADDR, DRIVE_LED);
        write_volatile(COL2_ADDR, DRIVE_LED);
        write_volatile(COL3_ADDR, DRIVE_LED);
        write_volatile(BTN_A_ADDR, DRIVE_INPUT);
        write_volatile(OUT_ADDR, out);
    }

    const IN_ADDR: *mut u32 = 0x5000_0510 as *mut u32;
    const BTNA_POS: u32 = 14;
    let mut btn_a: bool = false;

    loop {
        unsafe {
            btn_a = read_volatile(IN_ADDR) & 2u32.pow(BTNA_POS) != 2u32.pow(BTNA_POS);
        }

        if btn_a {
            let rnd_numbr = rng.random_range(1..7u32);
            show_number(rnd_numbr);
            for _ in 0..10_000 {
                nop();
            }
        } else {
            for _ in 0..500_000 {
                nop();
            }
        }
    }
}

fn show_number(number: u32) {
    const OUT_ADDR: *mut u32 = 0x5000_0504 as *mut u32;
    const ROW1_POS: u32 = 21;
    const COL1_POS: u32 = 28;
    const COL2_POS: u32 = 11;
    const COL3_POS: u32 = 31;
    // const POS_8:u32 = 28;

    let mut out: u32 = 1 << COL1_POS | 1 << COL2_POS | 1 << COL3_POS | 1 << ROW1_POS;
    if number & 4 == 4 {
        out = out ^ 1 << COL1_POS;
    }
    if number & 2 == 2 {
        out = out ^ 1 << COL2_POS;
    }
    if number & 1 == 1 {
        out = out ^ 1 << COL3_POS;
    }
    unsafe {
        write_volatile(OUT_ADDR, out);
    }
}
