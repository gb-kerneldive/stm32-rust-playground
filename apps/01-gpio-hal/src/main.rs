#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{
    pac,
    prelude::*,
    rcc::Config,
};

#[entry]
fn main() -> ! {

    let dp = pac::Peripherals::take()
                .expect("Périphériques STM32 déjà récupérés");
    let cp = cortex_m::peripheral::Peripherals::take()
                .expect("Périphériques Cortex-M déjà récupérés");
    
    let mut rcc = dp.RCC.freeze(Config::hsi().sysclk(48.MHz()));

    let gpiod = dp.GPIOD.split(&mut rcc);
    let mut led = gpiod.pd12.into_push_pull_output();

    let mut delay = cp.SYST.delay(&rcc.clocks);

    loop {
        led.toggle();
        delay.delay_ms(1000);

    }
}