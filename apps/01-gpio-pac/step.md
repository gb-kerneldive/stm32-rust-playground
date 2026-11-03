Controlling an LED on PD12 Using STM32 GPIO Registers
This guide explains how to enable the GPIOD peripheral clock, configure PD12 as a digital output, and turn an LED on or off by accessing STM32 registers directly.
Hardware note: The register addresses below apply to STM32 devices with this register map, such as the STM32F407. Check your microcontroller's reference manual and your board's schematic before using them. The LED behavior described assumes an active-high LED connected to PD12.

1. Enable the Peripheral Clock (RCC)
- Register: RCC_AHB1ENR
- Address: 0x40023830
- Action: Set bit 3 (GPIODEN) to 1 to enable the GPIOD peripheral clock.
- Bit mask: 0x00000008
2. Configure the Pin as an Output (MODER)
- Register: GPIOD_MODER
- Address: 0x40020C00
- Action: Set bits 25:24 to 01 to select general-purpose output mode for PD12.
- Bit operations: Clear bit 25 and set bit 24, preserving the other bits in the register.
3. Control the Output State (Turn the LED On or Off)
There are two ways to change the pin's output state.
Option A: Using the Output Data Register (ODR)
- Register: GPIOD_ODR
- Address: 0x40020C14
- Turn on: Set bit 12 to 1 (mask: 0x00001000).
- Turn off: Clear bit 12 to 0.
When using ODR, a read-modify-write operation is typically needed to change one pin without affecting the others.
Option B: Using the Bit Set/Reset Register (BSRR) — Recommended
- Register: GPIOD_BSRR
- Address: 0x40020C18
- Turn on: Write 1 to bit 12 (value: 0x00001000).
- Turn off: Write 1 to bit 28 (value: 0x10000000).
BSRR allows individual output bits to be set or reset with a single register write, without a read-modify-write operation. Write the indicated value directly to BSRR; do not try to clear its bits afterward.