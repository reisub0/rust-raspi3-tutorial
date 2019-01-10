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

use super::gpio::*;
use super::uarthandle;
use core::sync::atomic::{compiler_fence, Ordering};
use cortex_a::asm;

pub struct Led;

impl Led {
    pub fn new() -> Self {
        Self
    }
    pub fn init(&self) {
        unsafe {
            (*GPFSEL1).modify(GPFSEL1::FSEL16::Output);
            (*GPFSEL4).modify(GPFSEL4::FSEL47::Output);
            (*GPFSEL2).modify(GPFSEL2::FSEL29::Output);
        }
    }
    pub fn on(&self) {
        unsafe {
            (*GPSET0).write(GPSET0::O16::Set);
            (*GPSET0).write(GPSET0::O29::Set);
            (*GPSET1).write(GPSET1::O47::Set);
        }
    }
    pub fn off(&self) {
        unsafe {
            (*GPCLR0).write(GPCLR0::O16::Clear);
            (*GPCLR0).write(GPCLR0::O29::Clear);
            (*GPCLR1).write(GPCLR1::O47::Clear);
        }
    }
}
