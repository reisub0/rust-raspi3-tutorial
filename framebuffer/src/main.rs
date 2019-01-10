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

mod gpio;
mod lfb;
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

        uarthandle.getc(); // Press a key first before being greeted
        uarthandle.puts("Hello Rustadcean!\n");

        let mut fb = lfb::fb::new(&mut mbox);

        match fb {
            Ok(mut x) => x.cls(0xff1a1a),
            Err(_) => uarthandle.puts("Some error occured"),
        }

        // echo everything back
        loop {
            uarthandle.send(uarthandle.getc());
        }
    }
}
