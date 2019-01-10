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

use super::mbox;
use super::uarthandle;
use core::sync::atomic::{compiler_fence, Ordering};

pub struct fb {
    width: u32,
    height: u32,
    pitch: u32,
    ptr: usize,
}

impl fb {
    pub unsafe fn new(mbox: &mut mbox::Mbox) -> Result<Self, mbox::MboxError> {
        uarthandle.puts("In lfb new");
        mbox.buffer[0] = 35 * 4; // length of the message
        mbox.buffer[1] = mbox::REQUEST; // this is a request message

        mbox.buffer[2] = mbox::tag::SETPHYWH;
        mbox.buffer[3] = 8;
        mbox.buffer[4] = 8;
        mbox.buffer[5] = 1024;
        mbox.buffer[6] = 768;

        mbox.buffer[7] = mbox::tag::SETVIRTWH;
        mbox.buffer[8] = 8;
        mbox.buffer[9] = 8;
        mbox.buffer[10] = 1024;
        mbox.buffer[11] = 768;

        mbox.buffer[12] = mbox::tag::SETVIRTOFFSET;
        mbox.buffer[13] = 8;
        mbox.buffer[14] = 8;
        mbox.buffer[15] = 0;
        mbox.buffer[16] = 0;

        mbox.buffer[17] = mbox::tag::SETDEPTH;
        mbox.buffer[18] = 4;
        mbox.buffer[19] = 4;
        mbox.buffer[20] = 32;

        mbox.buffer[21] = mbox::tag::SETPXORDER;
        mbox.buffer[22] = 4;
        mbox.buffer[23] = 4;
        mbox.buffer[24] = 0;

        mbox.buffer[25] = mbox::tag::GETFB;
        mbox.buffer[26] = 8;
        mbox.buffer[27] = 8;
        mbox.buffer[28] = 4096;
        mbox.buffer[29] = 0;

        mbox.buffer[30] = mbox::tag::GETPITCH;
        mbox.buffer[31] = 4;
        mbox.buffer[32] = 4;
        mbox.buffer[33] = 0;

        mbox.buffer[34] = mbox::tag::LAST;

        uarthandle.puts("Calling mailbox with props");
        // compiler_fence(Ordering::Release);
        // send the message to the GPU and receive answer
        mbox.call(mbox::channel::PROP)?;
        uarthandle.puts("Done calling mailbox with props");
        Ok(Self {
            width: mbox.buffer[5],
            height: mbox.buffer[6],
            pitch: mbox.buffer[33],
            ptr: mbox.buffer[28] as usize,
        })
    }
    pub unsafe fn debug(&self) {
        uarthandle.puts("Self: {\r\n");
        uarthandle.puts("width: ");
        uarthandle.hex(self.width);
        uarthandle.puts("\r\n");
        uarthandle.puts("height: ");
        uarthandle.hex(self.height);
        uarthandle.puts("\r\n");
        uarthandle.puts("pitch: ");
        uarthandle.hex(self.pitch);
        uarthandle.puts("\r\n");
        uarthandle.puts("ptr: ");
        uarthandle.hex(self.ptr as u32);
        uarthandle.puts("\r\n");
        uarthandle.puts("}");
    }
    pub unsafe fn cls(&mut self, col: u32) {
        let mut cur = self.ptr as *mut u32;
        for _ in 0..self.height {
            for _ in 0..self.width {
                *cur = col;
                cur = cur.offset(1);
            }
        }
    }
}
