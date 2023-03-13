fn main() {
    println!("1 + 2 = {}", 1u32 + 2);

    println!("1 - 2 = {}", 1i32 - 2);

    // short-circuiting boolean logic
    println!("true AND false os {}", true && false);
    println!("true OR false os {}", true || false);
    println!("Not true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:4b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:4b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:4b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
