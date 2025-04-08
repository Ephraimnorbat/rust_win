Day of 4 learning Rust

Numbers - Integers
Integers are whole numbers.
Signed - can represnet both - & + integers
Unsigned -always positive integers

Length  - how many bytes an integer takes up

Length      Signed      Unsigned
8 bits       -i8      - u8
16 bits      -i16     - u16
32 bits      -i32     - u32
64 bits      -i64     - u64
128 bits     -i128    - u128
arch        -isize   - usize

Default types
Number - i32
Floating point - f64

 Binary number system
 42
 (4 x 10^1) + (2 x 10^0)
 =(4 x 10) + (2 x 1)
 =40 + 2
 =42

Day 5 of learning Rust

 What is a number
 Processor does NOT read 1 byte at a time from memory ,reads 1 word at a time.
 In a 32 bit processor a word is 4 bytes(32 bits).
 In a 64 bit processor a word is 8 bytes(64 bits)

usize gives you the guarantee to be always big enough to hold any pointer or any offset in a data structure.
Floating Point
f32 - size of 32 bits
f64 - size of 64 bits
Representation according to IEEE 754 specification.