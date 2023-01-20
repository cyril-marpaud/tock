// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2023
// Copyright Tock Contributors 2023

#![no_std]

pub use nrf52::{
    acomp, adc, aes, ble_radio, chip, clock, constants, crt1, deferred_call_tasks, ficr, i2c,
    ieee802154_radio, init, nvmc, peripheral_interrupts as base_interrupts, pinmux, power, ppi,
    pwm, rtc, spi, temperature, timer, trng, uart, uicr,
};
pub mod gpio;
pub mod interrupt_service;
