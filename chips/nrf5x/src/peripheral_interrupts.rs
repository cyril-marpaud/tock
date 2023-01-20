// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2023
// Copyright Tock Contributors 2023

pub const POWER_CLOCK: u32 = 0;
pub const RADIO: u32 = 1;
pub const UART0: u32 = 2;
pub const SPI0_TWI0: u32 = 3;
pub const SPI1_TWI1: u32 = 4;
#[cfg(feature = "nrf52")]
pub const NFCT: u32 = 5;
pub const GPIOTE: u32 = 6;
pub const ADC: u32 = 7;
pub const TIMER0: u32 = 8;
pub const TIMER1: u32 = 9;
pub const TIMER2: u32 = 10;
pub const RTC0: u32 = 11;
pub const TEMP: u32 = 12;
pub const RNG: u32 = 13;
pub const ECB: u32 = 14;
pub const CCM_AAR: u32 = 15;
pub const WDT: u32 = 16;
pub const RTC1: u32 = 17;
pub const QDEC: u32 = 18;
pub const COMP: u32 = 19;
pub const LPCOMP: u32 = 19;
pub const SWI0: u32 = 20;
pub const SWI1: u32 = 21;
pub const SWI2: u32 = 22;
pub const SWI3: u32 = 23;
pub const SWI4: u32 = 24;
pub const SWI5: u32 = 25;
#[cfg(feature = "nrf52")]
pub const TIMER3: u32 = 26;
#[cfg(feature = "nrf52")]
pub const TIMER4: u32 = 27;
#[cfg(feature = "nrf52")]
pub const PWM0: u32 = 28;
#[cfg(feature = "nrf52")]
pub const PDM: u32 = 29;
#[cfg(feature = "nrf52")]
pub const MWU: u32 = 32;
#[cfg(feature = "nrf52")]
pub const PWM1: u32 = 33;
#[cfg(feature = "nrf52")]
pub const PWM2: u32 = 34;
#[cfg(feature = "nrf52")]
pub const SPIM2_SPIS2_SPI2: u32 = 35;
#[cfg(feature = "nrf52")]
pub const RTC2: u32 = 36;
#[cfg(feature = "nrf52")]
pub const I2S: u32 = 37;
#[cfg(feature = "nrf52")]
pub const FPU: u32 = 38;
