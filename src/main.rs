#![no_std]
#![no_main]

use cortex_m_rt::entry;
use nrf52833_pac::{interrupt, Interrupt, NVIC, P0, TIMER0, TIMER1, TIMER2, TIMER3};
use panic_halt as _;

#[entry]
unsafe fn main() -> ! {
    // Turn on both LEDs.
    (&*P0::PTR).pin_cnf[21].write(|w| w.dir().output());
    (&*P0::PTR).pin_cnf[28].write(|w| w.dir().output());
    (&*P0::PTR).pin_cnf[11].write(|w| w.dir().output());
    (&*P0::PTR).outset.write(|w| w.pin21().set_bit()); // row 1
    (&*P0::PTR).outclr.write(|w| w.pin28().clear_bit_by_one()); // col 1
    (&*P0::PTR).outclr.write(|w| w.pin11().clear_bit_by_one()); // col 2

    // Set up periodic timers.

    (&*TIMER0::PTR).bitmode.write(|w| w.bitmode()._32bit());
    (&*TIMER0::PTR).cc[0].write(|w| w.cc().bits(400_000));
    (&*TIMER0::PTR).shorts.write(|w| w.compare0_clear().enabled());
    (&*TIMER0::PTR).intenset.write(|w| w.compare0().set());
    (&*NVIC::PTR).iser[0].write(1 << Interrupt::TIMER0 as u16);

    (&*TIMER1::PTR).bitmode.write(|w| w.bitmode()._32bit());
    (&*TIMER1::PTR).cc[0].write(|w| w.cc().bits(600_000));
    (&*TIMER1::PTR).shorts.write(|w| w.compare0_clear().enabled());
    (&*TIMER1::PTR).intenset.write(|w| w.compare0().set());
    (&*NVIC::PTR).iser[0].write(1 << Interrupt::TIMER1 as u16);

    (&*TIMER2::PTR).bitmode.write(|w| w.bitmode()._32bit());
    (&*TIMER2::PTR).cc[0].write(|w| w.cc().bits(200_000));
    (&*TIMER2::PTR).shorts.write(|w| w.compare0_clear().enabled());
    (&*TIMER2::PTR).intenset.write(|w| w.compare0().set());
    (&*NVIC::PTR).iser[0].write(1 << Interrupt::TIMER2 as u16);

    // Start timers 0 and 1.
    (&*TIMER0::PTR).tasks_start.write(|w| w.tasks_start().trigger());
    (&*TIMER1::PTR).tasks_start.write(|w| w.tasks_start().trigger());

    // busy wait 150_000 cycles
    (&*TIMER3::PTR).bitmode.write(|w| w.bitmode()._32bit());
    (&*TIMER3::PTR).cc[0].write(|w| w.cc().bits(150_000));
    (&*TIMER3::PTR).tasks_start.write(|w| w.tasks_start().trigger());
    while (&*TIMER3::PTR).events_compare[0].read().events_compare().is_not_generated() {}

    // Start timer 2.
    (&*TIMER2::PTR).tasks_start.write(|w| w.tasks_start().trigger());

    loop {}
}

#[interrupt]
unsafe fn TIMER0() {
    // Turn on LED 1.
    (&*P0::PTR).outclr.write(|w| w.pin28().clear_bit_by_one());

    (&*TIMER0::PTR).events_compare[0].write(|w| w.events_compare().not_generated());
    (&*TIMER0::PTR).events_compare[0].read().events_compare().is_generated();
    // ^ this read is necessary; see section 6.1.8 of the nRF52833 product spec.
}

#[interrupt]
unsafe fn TIMER1() {
    // Turn on LED 2.
    (&*P0::PTR).outclr.write(|w| w.pin11().clear_bit_by_one());

    (&*TIMER1::PTR).events_compare[0].write(|w| w.events_compare().not_generated());
    (&*TIMER1::PTR).events_compare[0].read().events_compare().is_generated();
}

#[interrupt]
unsafe fn TIMER2() {
    // Turn off both LEDs.
    (&*P0::PTR).outset.write(|w| w.pin28().set_bit());
    (&*P0::PTR).outset.write(|w| w.pin11().set_bit());

    (&*TIMER2::PTR).events_compare[0].write(|w| w.events_compare().not_generated());
    (&*TIMER2::PTR).events_compare[0].read().events_compare().is_generated();
}
