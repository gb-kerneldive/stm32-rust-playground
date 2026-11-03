#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4::stm32f407;

#[entry]
fn main() -> ! {
    // 1. Récupération des périphériques STM32
    let dp = stm32f407::Peripherals::take()
        .expect("Périphériques STM32 déjà récupérés");

    // 2. Récupération des périphériques du cœur Cortex-M4
    let cp = cortex_m::Peripherals::take()
        .expect("Périphériques Cortex-M déjà récupérés");

    // 3. Configuration du SysTick
    let mut delay = cortex_m::delay::Delay::new(cp.SYST, 16_000_000);

    // 4. Activation de l'horloge de GPIOD
    dp.RCC.ahb1enr.modify(|_, w| w.gpioden().set_bit());

    // 5. Configuration de PD12 en sortie
    dp.GPIOD.moder.modify(|_, w| w.moder12().output());

    loop {
        // LED verte allumée
        dp.GPIOD.bsrr.write(|w| w.bs12().set_bit());
        delay.delay_ms(2000_u32);

        // LED verte éteinte
        dp.GPIOD.bsrr.write(|w| w.br12().set_bit());
        delay.delay_ms(2000_u32);
    }
}
