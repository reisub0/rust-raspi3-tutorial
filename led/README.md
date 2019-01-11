# Tutorial ?? - LED

Rust is an amazing systems-level programming language that has _safety_ as one of its top design goals, along with _concurrency_ and _performance_. It is systems-level, meaning you can call assembly code within it, or work with raw memory contents. Rust feels and writes like a high level language, but it has all the powers of a lower level one like C. However, anything that has the potential to blow up in your face, like dereferencing a pointer that could be NULL, has to be wrapped up in `unsafe` code block, meaning you're telling the compiler that you know what you're doing. Note that this does **not** turn off the borrow checker.

Rust is heavily influenced by the likes of pure functional languages like Haskell while being pragmatic enough to not include a GC which would make it impractical for systems-level work. It borrows from the best aspects of these languages to create what is, in my opinion, the best language for low level work. The upfront cost and time investment for carefully defining your types and what data you're going to be working with might seem like too much of a price to pay compared to the 'Wild West' attitude of C. But the compiler really has got your back and you can be reasonably certain your code isn't going to blow up at runtime once it has passed the rigorous standards of the compiler. This does mean you will be fighting with the compiler a bit to get your code past it but it is worth it.

Especially when you're talking about bare metal programming where there's no debugging other than print statements (if you're lucky), it's almost impossible to know if the program you're writing in C is going to do what it's expected to do. Rust forces you to think about your code and your types and think about your error cases a bit more carefully. Once you've made it past the borrow checker and your code successfully compiles in Rust, you've also made it past so many pitfalls that the same code in C would have fallen into at runtime.

Rust is also strongly typed compared to weakly typed C. This means there's no unseen conversions to and from types. This means that another class of common errors from C are eliminated.

Enough ranting about programming languages, let's try to make something useful in Rust.

Here's what we're going to do: The Raspberry Pi 3 has an on-board Green LED that's usually mapped to SD Card activity by the operating system. Since we are going to implement the kernel, we're instead going to blink the so called `ACT` LED at regular intervals. And we're going to do this all from pure Rust.

To achieve this, you need a working knowledge of writing to memory mapped registers. Instead of mucking about with bitwise operators and shifts, we're going to rely on Rust's amazing macro and type support, and use the `register` crate to define the layout of our registers in memory. This enables us to write For similar code on how to define register contents, look around the excellent [rust-raspi3-tutorial](https://github.com/rust-embedded/rust-raspi3-tutorial) repository.

You will likely need to be _ARMed_ with the [BCM2837 ARM Peripherals Manual](https://www.raspberrypi.org/app/uploads/2012/02/BCM2835-ARM-Peripherals.pdf). The link is for the BCM2835. There is no difference between the two, except for a different base address. You'll be interested in Chapter 6: GPIO. Look around for what registers you need to modify to achieve what you want.

Some caveats to remember:

The base address is 0x3F00_0000 as opposed to the 0x7E00_0000 on the manual.

GPIO 29 is mapped to the ACT LED on the RPI 3B+.

If you're stuck at any point, you may look at my code for the same.

Good luck.
