#![no_std]
#![no_main]

use core::ptr::{read_volatile, write_volatile};
use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Initializing...");

    // Row 1 P0.21
    const ROW1_ADDR: *mut u32 = 0x5000_0754 as *mut u32;
    // Col 1 P0.28
    const COL1_ADDR: *mut u32 = 0x5000_0770 as *mut u32;
    const COL2_ADDR: *mut u32 = 0x5000_072c as *mut u32;
    const DIR_OUTPUT_POS: u32 = 0;
    const DRIVE_LED: u32 = 1 << DIR_OUTPUT_POS;

    // Btn A P0.14, unnecessary: should be 0 be default
    const BTN_A_ADDR: *mut u32 = 0x5000_0738 as *mut u32;
    const DRIVE_INPUT: u32 = 0 << DIR_OUTPUT_POS;

    const OUT_ADDR: *mut u32 = 0x5000_0504 as *mut u32;
    const ROW1_POS: u32 = 21;
    const COL1_POS: u32 = 28;
    const COL2_POS: u32 = 11;

    // Drive COLs high to deactivate LEDs and ROW to high to provide + on LEDS
    let mut out: u32 = 1 << COL1_POS | 1 << COL2_POS | 1 << ROW1_POS;

    unsafe {
        write_volatile(ROW1_ADDR, DRIVE_LED);
        write_volatile(COL1_ADDR, DRIVE_LED);
        write_volatile(COL2_ADDR, DRIVE_LED);
        write_volatile(BTN_A_ADDR, DRIVE_INPUT);
        write_volatile(OUT_ADDR, out);
    }

    const IN_ADDR: *mut u32 = 0x5000_0510 as *mut u32;
    const BTNA_POS: u32 = 14;
    let mut btn_a: bool = false;
    let mut ht: bool = false;

    loop {
        unsafe {
            btn_a = read_volatile(IN_ADDR) & 2u32.pow(BTNA_POS) != 2u32.pow(BTNA_POS);
        }

        if btn_a {
            if ht {
                out = out ^ 1 << COL1_POS;
                out = out | 1 << COL2_POS;
            } else {
                out = out | 1 << COL1_POS;
                out = out ^ 1 << COL2_POS;
            }

            unsafe {
                write_volatile(OUT_ADDR, out);
            }

            for _ in 0..10_000 {
                nop();
            }
            ht = !ht;
        } else {
            // Just slow down
            for _ in 0..100_000 {
                nop();
            }
        }
    }
}
