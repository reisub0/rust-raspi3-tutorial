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

#![no_std]
#![no_main]

extern crate cortex_a;

#[macro_use]
extern crate raspi3_boot;

#[macro_use]
extern crate register;

#[no_std]
#[macro_use]

const MMIO_BASE: u32 = 0x3F00_0000;

mod delays;
mod gpio;
mod led;
mod mbox;
mod uart;

use core::sync::atomic::{compiler_fence, Ordering};

entry!(kernel_entry);
static mut uarthandle: uart::Uart = uart::Uart {};

fn kernel_entry() -> ! {
    unsafe {
        uarthandle = uart::Uart::new();
        let mut mbox = mbox::Mbox::new();

        // set up serial console
        if uarthandle.init(&mut mbox).is_err() {
            loop {
                cortex_a::asm::wfe()
            } // If UART fails, abort early
        }

        uarthandle.puts("Pre new led\n");
        let led = led::Led::new();
        uarthandle.puts("Pre led init\n");
        led.init();
        uarthandle.puts("Post led init\n");
        loop {
            led.on();
            delays::wait_msec(500_000);
            // for _ in 0..1500000 {}
            led.off();
            delays::wait_msec(500_000);
            // for _ in 0..1500000 {}
        }
        loop {
            uarthandle.send(uarthandle.getc());
        }
    }
}
