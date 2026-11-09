//! Blink für das Infineon XMC4700 Relax (Lite) Kit – ohne PAC, direkter Registerzugriff.
//!
//! LEDs auf dem Board: LED1 = P5.9, LED2 = P5.8 (bitte im Schaltplan deiner
//! Board-Revision prüfen und ggf. unten anpassen).
#![no_std]
#![no_main]

use core::ptr::{read_volatile, write_volatile};
use cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _;

// Port 5 Basisadresse und Register-Offsets (XMC4700 Reference Manual, Kap. Ports)
const PORT5_BASE: usize = 0x4802_8500;
const OMR: usize = 0x04; // Output Modification Register
const IOCR8: usize = 0x18; // Input/Output Control Reg. für Pins 8..11

const LED1_PIN: u32 = 9;
const LED2_PIN: u32 = 8;

/// Push-Pull, General Purpose Output
const PC_PUSH_PULL: u32 = 0b10000;

#[inline(always)]
unsafe fn reg(offset: usize) -> *mut u32 {
    (PORT5_BASE + offset) as *mut u32
}

/// Pin n toggeln: PSn (Bit n) und PRn (Bit n+16) gleichzeitig setzen
#[inline(always)]
fn toggle(pin: u32) {
    unsafe { write_volatile(reg(OMR), (1 << pin) | (1 << (pin + 16))) }
}

/// Pin n auf Low setzen (LEDs sind active-low o. ä.: Zustand ggf. invertieren)
#[inline(always)]
fn set_low(pin: u32) {
    unsafe { write_volatile(reg(OMR), 1 << (pin + 16)) }
}

#[inline(always)]
fn set_high(pin: u32) {
    unsafe { write_volatile(reg(OMR), 1 << pin) }
}

#[entry]
fn main() -> ! {
    // P5.8 und P5.9 als Push-Pull-Ausgang konfigurieren
    // Pin 8 -> PC8 in Bits [7:3], Pin 9 -> PC9 in Bits [15:11] von IOCR8
    unsafe {
        let mut v = read_volatile(reg(IOCR8));
        v &= !((0x1F << 3) | (0x1F << 11));
        v |= (PC_PUSH_PULL << 3) | (PC_PUSH_PULL << 11);
        write_volatile(reg(IOCR8), v);
    }

    // Startzustand: LED1 an, LED2 aus -> wechseln sich beim Blinken ab
    set_high(LED1_PIN);
    set_low(LED2_PIN);

    loop {
        toggle(LED1_PIN);
        toggle(LED2_PIN);
        // Ungefähr 0,5 s bei der Reset-Taktung (nach Bedarf anpassen,
        // oder später SysTick / Timer nutzen)
        asm::delay(6_000_000);
    }
}
