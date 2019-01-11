/*
 * MIT License
 *
 * Copyright (c) 2018 Andre Richter <andre.o.richter@gmail.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use super::MMIO_BASE;
use core::ops;
use register::mmio::*;

// Descriptions taken from
// https://github.com/raspberrypi/documentation/files/1888662/BCM2837-ARM-Peripherals.-.Revised.-.V2-1.pdf
register_bitfields! {
    u32,

    GPFSEL2 [
        FSEL29 OFFSET(27) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ]
    ],

    GPSET0 [
        O29 OFFSET(29) NUMBITS(1) [
            Clear = 0,
            Set = 1
        ]
    ],

    GPCLR0 [
        O29 OFFSET(29) NUMBITS(1) [
            Set = 0,
            Clear = 1
        ]
    ]
}

const GPIO_BASE: u32 = MMIO_BASE + 0x20_0000;

#[allow(non_snake_case)]
#[repr(C)]
pub struct RegisterBlock {
    __GPFSEL0: u32,                                 // 0x00
    __GPFSEL1: u32,                                 // 0x04
    pub GPFSEL2: ReadWrite<u32, GPFSEL2::Register>, // 0x08
    __GPFSEL3: u32,                                 // 0x0C
    __GPFSEL4: u32,                                 // 0x10
    __GPFSEL5: u32,                                 // 0x14
    __reserved_0: u32,                              // 0x18
    pub GPSET0: ReadWrite<u32, GPSET0::Register>,   // 0x1C
    __GPSET1: u32,                                  // 0x20
    __reserved_1: u32,                              // 0x24
    pub GPCLR0: ReadWrite<u32, GPCLR0::Register>,   // 0x28
}

pub struct Gpio;

impl ops::Deref for Gpio {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

impl Gpio {
    /// Returns a pointer to the register block
    fn ptr() -> *const RegisterBlock {
        GPIO_BASE as *const _
    }
}
