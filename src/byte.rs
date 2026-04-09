
// Converts s8 to an unsigned byte
#[inline]
pub fn to_u8(s8: i32) -> u8
{
    (s8 & 0xFF) as u8
}

/// Converts `u8` to a signed byte (254 => -2).
#[inline]
pub fn to_s8(u8: u8) -> i8 {
    u8 as i8
}

/// Forces a `value` to fit in 16 bits (65537 => 1).
#[inline]
pub fn to_u16(value: i32) -> u16 {
    (value & 0xFFFF) as u16
}

/// Returns whether `u8` can be represented as a single byte or not.
#[inline]
pub fn overflows(u8: u32) -> bool {
    u8 >= 256
}

/// Returns whether `s8` is positive or not.
#[inline]
pub fn is_positive(s8: u8) -> bool {
    (s8 >> 7) & 1 == 0
}

/// Returns whether `s8` is negative or not.
#[inline]
pub fn is_negative(s8: u8) -> bool {
    (s8 >> 7) & 1 == 1
}

/// Returns the bit located at `position` in `number`, as a boolean.
#[inline]
pub fn get_flag(number: u8, position: u8) -> bool {
    get_bit(number, position) != 0
}

/// Returns the bit located at `position` in `number`.
#[inline]
pub fn get_bit(number: u8, position: u8) -> u8 {
    (number >> position) & 1
}

/// Returns an updated `u8`, with a `bit` changed to `value` (0 or 1).
#[inline]
pub fn set_bit(u8: u8, bit: u8, value: u8) -> u8 {
    let mask = 1u8 << bit;
    (u8 & !mask) | ((value & 0b1) << bit)
}

/// Returns a sub-number of `size` bits inside `u8`, starting at `start_position`.
#[inline]
pub fn get_bits(u8: u8, start_position: u8, size: u8) -> u8 {
    (u8 >> start_position) & (0xFF >> (8 - size))
}

/// Inserts a `value` of `size` bits inside `u8`, starting at `start_position`.
/// Returns the updated number.
#[inline]
pub fn set_bits(u8: u8, start_position: u8, size: u8, value: u8) -> u8 {
    let mask = ((1u8 << size) - 1) << start_position;
    (u8 & !mask) | ((value << start_position) & mask)
}

/// Returns the most significant byte of `u16`.
#[inline]
pub fn high_byte_of(u16: u16) -> u8 {
    (u16 >> 8) as u8
}

/// Returns the least significant byte of `u16`.
#[inline]
pub fn low_byte_of(u16: u16) -> u8 {
    (u16 & 0xFF) as u8
}

/// Returns a 16-bit number from `high_byte` and `low_byte`.
#[inline]
pub fn build_u16(high_byte: u8, low_byte: u8) -> u16 {
    ((high_byte as u16) << 8) | (low_byte as u16)
}

/// Returns the upper nybble of `u8`.
#[inline]
pub fn high_nybble_of(u8: u8) -> u8 {
    u8 >> 4
}

/// Returns the lower nybble of `u8`.
#[inline]
pub fn low_nybble_of(u8: u8) -> u8 {
    u8 & 0b1111
}

/// Returns an 8-bit number from `high_nybble` and `low_nybble`.
#[inline]
pub fn build_u8(high_nybble: u8, low_nybble: u8) -> u8 {
    ((high_nybble & 0b1111) << 4) | (low_nybble & 0b1111)
}

/// Returns an 8-bit number from `bit0`, `bit1`, `bit2`, etc.
#[inline]
pub fn bitfield(bit0: u8, bit1: u8, bit2: u8, bit3: u8,
                bit4: u8, bit5: u8, bit6: u8, bit7: u8) -> u8 {
    ((bit0 & 1) << 0) |
    ((bit1 & 1) << 1) |
    ((bit2 & 1) << 2) |
    ((bit3 & 1) << 3) |
    ((bit4 & 1) << 4) |
    ((bit5 & 1) << 5) |
    ((bit6 & 1) << 6) |
    ((bit7 & 1) << 7)
}

/// Returns a 2-bit number from `high_bit` and `low_bit`.
#[inline]
pub fn build_u2(high_bit: u8, low_bit: u8) -> u8 {
    (high_bit << 1) | low_bit
}
