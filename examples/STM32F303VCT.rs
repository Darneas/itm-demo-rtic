#![deny(unsafe_code)]
#![no_main]
#![no_std]

pub use cortex_m::{asm, iprintln};
use stm32f3_discovery::stm32f3xx_hal as hal;
use hal::stm32 as target;
use panic_semihosting as _;

#[rtic::app(device = stm32f3_discovery::stm32f3xx_hal::stm32, peripherals = true)]
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
            cortex_m::asm::delay(1000000);
        }
    }
};
