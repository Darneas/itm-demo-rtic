#![deny(unsafe_code)]
#![no_main]
#![no_std]

pub use cortex_m::{asm, iprintln};
use nrf52840_hal;
use nrf52840_hal::pac as target;
use panic_semihosting as _;

#[rtic::app(device = nrf52840_hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        itm: target::ITM,
    }
    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        let cp = cx.core;
        let mut itm = cp.ITM;

        iprintln!(&mut itm.stim[0], "init");
        init::LateResources { itm }
    }

    #[idle(resources = [itm])]
    fn idle(cx: idle::Context) -> ! {
        let itm = cx.resources.itm;
        loop {
            iprintln!(&mut itm.stim[0], "idle");
            asm::wfi();
        }
    }
};
