#![deny(unsafe_code)]
#![no_main]
#![no_std]
#![allow(non_snake_case)]

pub use cortex_m::{asm, iprintln};
use hal::pac as target;
use panic_semihosting as _;
use stm32f1xx_hal as hal;

#[rtic::app(device = stm32f1xx_hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        itm: target::ITM,
    }
    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        rtt_target::rtt_init_print!();
        let cp = cx.core;
        let mut itm = cp.ITM;

        iprintln!(&mut itm.stim[0], "init");
        init::LateResources { itm }
    }

    #[idle(resources = [itm])]
    fn idle(cx: idle::Context) -> ! {
        let itm = cx.resources.itm;
        loop {
            rtt_target::rprintln!("Hello, world!");
            iprintln!(&mut itm.stim[0], "idle");
            cortex_m::asm::delay(1000000);
        }
    }
};
