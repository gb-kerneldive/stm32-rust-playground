#![no_std]
#![no_main]

use cortex_m_rt::entry;
use pac::interrupt;
use panic_halt as _;
use stm32f4::stm32f407 as pac;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // 0. Keep the debug interface alive in sleep/stop/standby, so that WFI
    //    never locks out SWD (otherwise flashing fails with "SwdDpWait").
    //    Garder le debug actif en veille, pour que WFI ne bloque pas le SWD.
    dp.DBGMCU.cr.modify(|_, w| {
        w.dbg_sleep().set_bit();
        w.dbg_stop().set_bit();
        w.dbg_standby().set_bit()
    });

    // 1. Clocks: GPIOA (button), GPIOD (LED), SYSCFG (EXTI routing)
    //    Horloges : GPIOA (bouton), GPIOD (LED), SYSCFG (routage EXTI)
    dp.RCC
        .ahb1enr
        .modify(|_, w| w.gpioaen().set_bit().gpioden().set_bit());
    dp.RCC.apb2enr.modify(|_, w| w.syscfgen().set_bit());

    // 2. PA0 = input, PD12 = output (SWD pins PA13/PA14 are left untouched)
    //    PA0 = entrée, PD12 = sortie (PA13/PA14 du SWD non touchés)
    dp.GPIOA.moder.modify(|_, w| w.moder0().input());
    dp.GPIOD.moder.modify(|_, w| w.moder12().output());

    // 3. Route PA0 to EXTI line 0
    //    Router PA0 vers la ligne EXTI 0
    dp.SYSCFG
        .exticr1
        .modify(|_, w| unsafe { w.exti0().bits(0) });

    // 4. EXTI line 0: rising + falling edge, clear stale flag, unmask
    //    EXTI ligne 0 : deux fronts, effacer un flag résiduel, démasquer
    dp.EXTI.rtsr.modify(|_, w| w.tr0().set_bit());
    dp.EXTI.ftsr.modify(|_, w| w.tr0().set_bit());
    dp.EXTI.pr.write(|w| w.pr0().set_bit());
    dp.EXTI.imr.modify(|_, w| w.mr0().set_bit());

    // 5. Enable the interrupt in the NVIC
    //    Activer l'interruption dans le NVIC
    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::EXTI0);
    }

    loop {
        cortex_m::asm::wfi();
    }
}

#[interrupt]
fn EXTI0() {
    // SAFETY: after start-up, main only sleeps; the handler is the only user
    // of EXTI.PR, GPIOA.IDR (read) and GPIOD.BSRR (write-only).
    // Après l'initialisation, main dort ; le handler est seul à utiliser ces registres.
    let dp = unsafe { pac::Peripherals::steal() };

    // Clear the flag first (write 1 to clear), so no edge is lost
    // Effacer le flag d'abord (écrire 1), pour ne perdre aucun front
    dp.EXTI.pr.write(|w| w.pr0().set_bit());

    // Discovery user button: pressed = PA0 high
    // Bouton utilisateur Discovery : appuyé = PA0 à l'état haut
    if dp.GPIOA.idr.read().idr0().bit_is_set() {
        dp.GPIOD.bsrr.write(|w| w.bs12().set_bit()); // LED on
    } else {
        dp.GPIOD.bsrr.write(|w| w.br12().set_bit()); // LED off
    }
}
