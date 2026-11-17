#![no_std]
#![no_main]

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use pac::{interrupt, Peripherals};
use panic_halt as _;
use stm32f4::stm32f407 as pac;

// Peripherals shared between main and the interrupt handler.
// Périphériques partagés entre main et le handler d'interruption.
static G_PER: Mutex<RefCell<Option<Peripherals>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().expect("Périphériques STM32 déjà récupérés");

    // 1. Enable GPIOA, GPIOD and SYSCFG clocks
    //    Activer les horloges GPIOA, GPIOD et SYSCFG
    dp.RCC.ahb1enr.modify(|_, w| w.gpioaen().set_bit().gpioden().set_bit());
    dp.RCC.apb2enr.modify(|_, w| w.syscfgen().set_bit());

    // 2. Configure PA0 as input (0b00) and PD12 as output
    //    Configurer PA0 en entrée (0b00) et PD12 en sortie
    dp.GPIOA.moder.modify(|_, w| w.moder0().input());
    dp.GPIOD.moder.modify(|_, w| w.moder12().output());

    // 3. SYSCFG routing: 0 selects Port A for EXTI line 0 (PA0)
    //    Routage dans le SYSCFG : 0 pour le Port A (PA0)
    dp.SYSCFG.exticr1.modify(|_, w| unsafe { w.exti0().bits(0) });

    // 4. EXTI configuration: rising edge + unmask the line (IMR)
    //    Configuration de l'EXTI : front montant + démasquage (IMR)
    dp.EXTI.rtsr.modify(|_, w| w.tr0().set_bit());
    dp.EXTI.imr.modify(|_, w| w.mr0().set_bit());

    // 5. Share the peripherals with the handler BEFORE enabling the IRQ
    //    Partager les périphériques avec le handler AVANT d'activer l'IRQ
    cortex_m::interrupt::free(|cs| {
        G_PER.borrow(cs).replace(Some(dp));
    });

    // 6. Enable the interrupt in the NVIC
    //    Activer l'interruption dans le NVIC
    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::EXTI0);
    }

    loop {
        // Wait for interrupt (saves power) - currently disabled
        // Attente d'interruption (économise l'énergie) - actuellement désactivé
        //cortex_m::asm::wfi();
    }
}


#[interrupt]
fn EXTI0() {
    cortex_m::interrupt::free(|cs| {
        // Get access to the global peripherals
        // Obtenir l'accès aux périphériques globaux
        let mut dp = G_PER.borrow(cs).borrow_mut();
        if dp.as_mut().unwrap().EXTI.pr.read().pr0().bit() {
            // Clear the button interrupt flag (write 1 to clear)
            // Effacer le flag d'interruption du bouton (écrire 1 pour effacer)
            dp.as_mut().unwrap().EXTI.pr.modify(|_, w| w.pr0().set_bit());
            // Turn the LED on (PD12)
            // Allumer la LED (PD12)
            dp.as_mut()
                .unwrap()
                .GPIOD
                .bsrr
                .write(|w| w.bs12().set_bit());
        }
    });
}
