#![no_std]
#![no_main]

// pick a panicking behavior
use panic_semihosting as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                            // use panic_abort as _; // requires nightly
                            // use panic_itm as _; // logs messages over ITM; requires ITM support
                            // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::{iprintln, itm};
use cortex_m_rt::entry;
use nrf52840_hal::gpio::{p0::Parts, Level};
use nrf52840_hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut core = cortex_m::Peripherals::take().unwrap();
    let nrf = nrf52840_hal::pac::Peripherals::take().unwrap();

    let p0 = Parts::new(nrf.P0);
    let led1 = p0.p0_14;

    let mut itm = core.ITM;
    let tpiu = core.TPIU;
    unsafe {
        core.DCB.enable_trace();

        tpiu.cspsr.write(1); // port size = 1

        tpiu.acpr.write(3); // divider = 4
        tpiu.sppr.write(2); // NRZ encoding
        tpiu.ffcr.write(0x100);

        itm.lar.write(0xc5ac_ce55);
        itm.tcr.write(0x0001_0005);
        itm.ter[0].write(3);
        itm.tpr.write(1);
    }

    nrf.CLOCK
        .traceconfig
        .write(|w| w.traceportspeed()._4mhz().tracemux().serial());
    let traceconfig_init = nrf.CLOCK.traceconfig.read().bits();

    itm::write_str(&mut itm.stim[0], "Hello World!\n");

    let mut led1 = led1.into_push_pull_output(Level::Low);
    led1.set_low().unwrap();

    loop {
        let traceconfig = nrf.CLOCK.traceconfig.read().bits();
        iprintln!(
            &mut itm.stim[0],
            "TRACECONFIG=0x{:08X} (INIT=0x{:08X}), TPIU.CSPSR=0x{:08X}, TPIU.ACPR=0x{:08X}",
            traceconfig,
            traceconfig_init,
            tpiu.cspsr.read(),
            tpiu.acpr.read(),
        );
    }
}
